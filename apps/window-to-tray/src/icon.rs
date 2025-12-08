use {
    crate::{
        desktop::get_icon_name,
        wtt::{Color, Globals},
    },
    error_reporter::Report,
    ini::{Ini, ParseError},
    isnt::std_1::primitive::IsntSliceExt,
    png::Transformations,
    resvg::{
        tiny_skia::{PixmapMut, Transform},
        usvg::{self, Options, Tree},
    },
    std::{
        collections::{HashMap, HashSet},
        env::var,
        io::{self, Cursor, Write},
        mem,
        os::unix::ffi::OsStrExt,
        path::{Path, PathBuf},
        rc::Rc,
        str::FromStr,
        sync::{Arc, LazyLock},
    },
    thiserror::Error,
    uapi::c,
    wl_proxy::{
        object::ObjectCoreApi,
        protocols::wayland::{wl_buffer::WlBuffer, wl_shm::WlShmFormat},
    },
};

pub struct IconTemplate {
    version: usize,
    name: Option<String>,
    app_id: Option<String>,
    buffers: Vec<Rc<BufferIconFrame>>,
    theme: Arc<String>,
    color: Color,
}

#[derive(Default, Eq, PartialEq)]
struct IconVersion {
    version: usize,
    nominal_size: [i32; 2],
    buffer_size: [i32; 2],
    scale: i32,
}

pub struct BufferIconFrame {
    pub buffer: Rc<WlBuffer>,
    pub buffer_size: [i32; 2],
    pub scale: i32,
}

pub struct BufferIcon {
    version: IconVersion,
    buffer: Rc<BufferIconFrame>,
}

impl Drop for BufferIconFrame {
    fn drop(&mut self) {
        self.buffer.send_destroy();
    }
}

impl IconTemplate {
    pub fn new(theme: &Arc<String>, color: &Color) -> Self {
        Self {
            version: 1,
            name: Default::default(),
            app_id: Default::default(),
            buffers: Default::default(),
            theme: theme.clone(),
            color: *color,
        }
    }

    pub fn update_app_id(&mut self, app_id: Option<&str>) {
        if self.app_id.as_deref() == app_id {
            return;
        }
        self.version += 1;
        self.app_id = app_id.map(|s| s.to_string());
    }

    pub fn update_name(&mut self, name: Option<String>) {
        if self.name == name {
            return;
        }
        self.version += 1;
        self.name = name;
    }

    pub fn update_buffers(&mut self, buffers: Vec<Rc<BufferIconFrame>>) {
        self.version += 1;
        self.buffers = buffers;
    }

    fn realize(
        &self,
        g: &Globals,
        nominal_size: [i32; 2],
        buffer_size: [i32; 2],
        scale: i32,
    ) -> Rc<BufferIconFrame> {
        if self.buffers.is_not_empty() {
            let find_best = |force_scale: bool| {
                let mut res = None;
                let mut max_dist = i32::MAX;
                for buffer in &self.buffers {
                    if force_scale && buffer.scale != scale {
                        continue;
                    }
                    let dist = (buffer.buffer_size[0] - buffer_size[0]).pow(2)
                        + (buffer.buffer_size[1] - buffer_size[1]).pow(2)
                        + (buffer.scale - scale).pow(2);
                    if dist < max_dist {
                        max_dist = dist;
                        res = Some(buffer);
                    }
                }
                res
            };
            if let Some(res) = find_best(true) {
                return res.clone();
            }
            if let Some(res) = find_best(false) {
                return res.clone();
            }
            return self.buffers[0].clone();
        }
        let (bytes, buffer_size) = self.realize_shm(nominal_size, buffer_size, scale);
        match create_shm_buf(g, &bytes, buffer_size) {
            Ok(buffer) => Rc::new(BufferIconFrame {
                buffer,
                buffer_size,
                scale,
            }),
            Err(e) => {
                log::error!("Could not create shm buffer: {}", Report::new(e));
                g.black_frame.clone()
            }
        }
    }

    fn realize_shm(
        &self,
        nominal_size: [i32; 2],
        buffer_size: [i32; 2],
        scale: i32,
    ) -> (Vec<u8>, [i32; 2]) {
        let mut name = self.name.as_deref();
        if name.is_none()
            && let Some(app_id) = &self.app_id
        {
            name = get_icon_name(app_id);
        }
        if let Some(name) = name {
            let res = name_to_bytes(
                name,
                nominal_size,
                buffer_size,
                scale,
                &self.theme,
                &self.color,
            );
            if let Some(res) = res {
                return res;
            }
        }
        let mut data = match render_svg(include_bytes!("fallback.svg"), buffer_size, &self.color) {
            Ok(d) => d,
            Err(e) => {
                log::error!("Could not render fallback: {}", Report::new(e));
                vec![255; (buffer_size[0] * buffer_size[1]) as usize]
            }
        };
        to_bgra(&mut data);
        (data, buffer_size)
    }
}

impl IconVersion {
    fn update(
        &mut self,
        template: &IconTemplate,
        nominal_size: [i32; 2],
        buffer_size: [i32; 2],
        scale: i32,
    ) -> bool {
        let version = Self {
            version: template.version,
            nominal_size,
            buffer_size,
            scale,
        };
        if self == &version {
            return true;
        }
        *self = version;
        false
    }
}

impl BufferIcon {
    pub fn new(g: &Globals) -> Self {
        Self {
            version: Default::default(),
            buffer: g.black_frame.clone(),
        }
    }

    pub fn get(&self) -> &Rc<WlBuffer> {
        &self.buffer.buffer
    }

    pub fn update(
        &mut self,
        template: &IconTemplate,
        nominal_size: [i32; 2],
        buffer_size: [i32; 2],
        scale: i32,
        g: &Globals,
    ) -> bool {
        if self
            .version
            .update(template, nominal_size, buffer_size, scale)
        {
            return false;
        }
        self.buffer = template.realize(g, nominal_size, buffer_size, scale);
        true
    }
}

fn name_to_bytes(
    name: &str,
    nominal_size: [i32; 2],
    buffer_size: [i32; 2],
    scale: i32,
    theme: &str,
    color: &Color,
) -> Option<(Vec<u8>, [i32; 2])> {
    let lookup = find_icon(name, nominal_size[0].max(nominal_size[1]), scale, theme)?;
    let contents = match std::fs::read(&lookup.path) {
        Ok(c) => c,
        Err(e) => {
            log::error!(
                "Could not read {}: {}",
                lookup.path.display(),
                Report::new(e)
            );
            return None;
        }
    };
    let ext = lookup.path.extension()?;
    let (mut contents, buffer_size) = match ext.as_bytes() {
        b"svg" => match render_svg(&contents, buffer_size, color) {
            Ok(b) => (b, buffer_size),
            Err(e) => {
                log::error!("Could not render svg: {}", Report::new(e));
                return None;
            }
        },
        b"png" => match render_png(&contents) {
            Ok(b) => b,
            Err(e) => {
                log::error!("Could not render png: {}", Report::new(e));
                return None;
            }
        },
        _ => return None,
    };
    to_bgra(&mut contents);
    Some((contents, buffer_size))
}

fn to_bgra(contents: &mut [u8]) {
    let mut chunks = contents.chunks_mut(4);
    while let Some([r, g, b, a]) = chunks.next() {
        // Convert to premultiplied BGRA.
        mem::swap(r, b);
        *r = (*r as f32 * *a as f32 / 255.0) as u8;
        *g = (*g as f32 * *a as f32 / 255.0) as u8;
        *b = (*b as f32 * *a as f32 / 255.0) as u8;
    }
}

fn render_svg(
    contents: &[u8],
    buffer_size: [i32; 2],
    color: &Color,
) -> Result<Vec<u8>, usvg::Error> {
    let map = |c: f32| (c * 255.0).round();
    let stylesheet = format!(
        "* {{ color: rgb({} {} {} {}); }}",
        map(color.r / color.a),
        map(color.g / color.a),
        map(color.b / color.a),
        map(color.a)
    );
    let options = Options {
        style_sheet: Some(stylesheet),
        ..Default::default()
    };
    let tree = Tree::from_data(contents, &options)?;
    let mut res = vec![0; (buffer_size[0] * buffer_size[1] * 4) as usize];
    let mut pixmap = PixmapMut::from_bytes(&mut res, buffer_size[0] as _, buffer_size[1] as _)
        .expect("Could not create PixmapMut");
    let actual = tree.size();
    let transform = Transform::from_scale(
        buffer_size[0] as f32 / actual.width(),
        buffer_size[1] as f32 / actual.height(),
    );
    resvg::render(&tree, transform, &mut pixmap);
    Ok(res)
}

fn render_png(contents: &[u8]) -> Result<(Vec<u8>, [i32; 2]), png::DecodingError> {
    let mut contents = Cursor::new(contents);
    let mut decoder = png::Decoder::new(&mut contents);
    decoder.set_transformations(Transformations::STRIP_16 | Transformations::ALPHA);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size().unwrap_or(0)];
    let info = reader.next_frame(&mut buf)?;
    Ok((buf, [info.width as _, info.height as _]))
}

#[derive(Debug)]
struct IconLookup {
    path: PathBuf,
}

fn find_icon(name: &str, nominal_size: i32, scale: i32, theme: &str) -> Option<IconLookup> {
    if (name.ends_with(".png") || name.ends_with("svg"))
        && let Ok(m) = std::fs::metadata(name)
        && m.is_file()
    {
        return Some(IconLookup {
            path: Path::new(name).to_path_buf(),
        });
    }
    find_icon_within(
        ICON_BASE_DIRS.iter().map(|d| &**d),
        &THEMES,
        name,
        nominal_size,
        scale,
        theme,
    )
}

fn find_icon_within<'a, I>(
    base_dirs: I,
    themes: &HashMap<String, Vec<Theme>>,
    name: &str,
    nominal_size: i32,
    scale: i32,
    theme: &str,
) -> Option<IconLookup>
where
    I: IntoIterator<Item = &'a Path>,
{
    let mut searched_themes = HashSet::new();
    let mut res = find_icon_helper(
        themes,
        name,
        nominal_size,
        scale,
        theme,
        &mut searched_themes,
    );
    if res.is_none() {
        res = find_icon_helper(
            themes,
            name,
            nominal_size,
            scale,
            "Hicolor",
            &mut searched_themes,
        );
    }
    if res.is_none() {
        for dir in base_dirs {
            res = find_icon_in_dir(dir, "", name).map(|path| IconLookup { path });
            if res.is_some() {
                break;
            }
        }
    }
    res
}

fn find_icon_helper<'a>(
    themes: &'a HashMap<String, Vec<Theme>>,
    name: &str,
    nominal_size: i32,
    scale: i32,
    theme_name: &'a str,
    searched_themes: &mut HashSet<&'a str>,
) -> Option<IconLookup> {
    if !searched_themes.insert(theme_name) {
        return None;
    }
    for theme in themes.get(theme_name)? {
        let res = lookup_icon(name, nominal_size, scale, theme);
        if res.is_some() {
            return res;
        }
        for parent in &theme.inherits {
            let res = find_icon_helper(themes, name, nominal_size, scale, parent, searched_themes);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

fn lookup_icon(name: &str, nominal_size: i32, scale: i32, theme: &Theme) -> Option<IconLookup> {
    for dir in &theme.directories {
        if let Some(variant) = theme.variants.get(dir)
            && variant.permits_size(nominal_size, scale)
            && let Some(path) = find_icon_in_dir(&theme.dir, dir, name)
        {
            return Some(IconLookup { path });
        }
    }
    let mut min_dist = i32::MAX;
    let mut closest = None;
    for dir in &theme.directories {
        if let Some(variant) = theme.variants.get(dir) {
            let dist = variant.distance(nominal_size, scale);
            if dist >= min_dist {
                continue;
            }
            if let Some(path) = find_icon_in_dir(&theme.dir, dir, name) {
                min_dist = dist;
                closest = Some(IconLookup { path });
            }
        }
    }
    closest
}

fn find_icon_in_dir(dir: &Path, subdir: &str, name: &str) -> Option<PathBuf> {
    const EXTENSIONS: [&str; 2] = ["svg", "png"];
    for ext in EXTENSIONS {
        let path = dir.join(format!("./{subdir}/{name}.{ext}"));
        if path.exists() {
            return Some(path);
        }
    }
    None
}

impl Variant {
    fn permits_size(&self, nominal_size: i32, scale: i32) -> bool {
        if self.scale != scale {
            return false;
        }
        match self.ty {
            VariantType::Threshold => {
                self.nominal_size - self.threshold <= nominal_size
                    && nominal_size <= self.nominal_size + self.threshold
            }
            VariantType::Scalable => {
                self.min_nominal_size <= nominal_size && nominal_size <= self.max_nominal_size
            }
            VariantType::Fixed => self.nominal_size == nominal_size,
        }
    }

    fn distance(&self, nominal_size: i32, scale: i32) -> i32 {
        match self.ty {
            VariantType::Threshold => {
                if nominal_size * scale < (self.nominal_size - self.threshold) * self.scale {
                    return self.min_nominal_size * self.scale - nominal_size * scale;
                }
                if nominal_size * scale > (self.nominal_size + self.threshold) * self.scale {
                    return nominal_size * scale - self.max_nominal_size * self.scale;
                }
                0
            }
            VariantType::Scalable => {
                if nominal_size * scale < self.min_nominal_size * self.scale {
                    return self.min_nominal_size * self.scale - nominal_size * scale;
                }
                if nominal_size * scale > self.max_nominal_size * self.scale {
                    return nominal_size * scale - self.max_nominal_size * self.scale;
                }
                0
            }
            VariantType::Fixed => (self.nominal_size * self.scale - nominal_size * scale).abs(),
        }
    }
}

static THEMES: LazyLock<HashMap<String, Vec<Theme>>> = LazyLock::new(|| {
    let mut themes = HashMap::<_, Vec<_>>::new();
    for dir in &*ICON_BASE_DIRS {
        parse_themes_in_dir(dir, &mut themes);
    }
    themes
});

fn parse_themes_in_dir(dir: &Path, out: &mut HashMap<String, Vec<Theme>>) {
    let Ok(mut dir) = dir.read_dir() else {
        return;
    };
    while let Some(Ok(dir)) = dir.next() {
        let path = dir.path();
        let res = parse_theme(&path);
        if let Some(res) = res.transpose() {
            match res {
                Ok(theme) => {
                    out.entry(theme.name.clone()).or_default().push(theme);
                }
                Err(e) => {
                    log::debug!(
                        "Could not parse theme in {}: {}",
                        path.display(),
                        Report::new(e)
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
struct Theme {
    name: String,
    dir: PathBuf,
    _comment: Option<String>,
    inherits: Vec<String>,
    directories: Vec<String>,
    variants: HashMap<String, Variant>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VariantType {
    Threshold,
    Scalable,
    Fixed,
}

#[derive(Debug)]
struct Variant {
    nominal_size: i32,
    scale: i32,
    ty: VariantType,
    max_nominal_size: i32,
    min_nominal_size: i32,
    threshold: i32,
}

#[derive(Debug, Error)]
enum ThemeError {
    #[error("Could not parse the theme file")]
    Parse(#[source] ParseError),
    #[error("The theme has no name")]
    NoName,
}

fn parse_theme(dir: &Path) -> Result<Option<Theme>, ThemeError> {
    let file = dir.join("index.theme");
    let Ok(theme) = std::fs::read_to_string(&file) else {
        return Ok(None);
    };
    let mut ini = Ini::load_from_str(&theme).map_err(ThemeError::Parse)?;
    let Some(desc) = ini.delete(Some("Icon Theme")) else {
        return Ok(None);
    };
    let split = |name: &str| {
        desc.get(name)
            .unwrap_or_default()
            .split(",")
            .map(ToOwned::to_owned)
    };
    let mut theme = Theme {
        name: desc.get("Name").ok_or(ThemeError::NoName)?.to_string(),
        dir: dir.to_owned(),
        _comment: desc.get("Comment").map(ToOwned::to_owned),
        inherits: split("Inherits").collect(),
        directories: split("Directories")
            .chain(split("ScaledDirectories"))
            .collect(),
        variants: Default::default(),
    };
    for (section, props) in ini.iter() {
        let Some(section) = section else {
            continue;
        };
        let Some(nominal_size) = props.get("Size") else {
            continue;
        };
        let Ok(nominal_size) = i32::from_str(nominal_size) else {
            continue;
        };
        let ty = match props.get("Type") {
            None | Some("Threshold") => VariantType::Threshold,
            Some("Scalable") => VariantType::Scalable,
            Some("Fixed") => VariantType::Fixed,
            Some(ty) => {
                log::error!("In {}: Unknown Type {}", dir.display(), ty);
                continue;
            }
        };
        macro_rules! int {
            ($name:expr, $default:expr) => {
                match props.get($name) {
                    None => $default,
                    Some(v) => match i32::from_str(v) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!(
                                "In {}: Could not parse {}: {}",
                                dir.display(),
                                $name,
                                Report::new(e)
                            );
                            continue;
                        }
                    },
                }
            };
        }
        theme.variants.insert(
            section.to_string(),
            Variant {
                nominal_size,
                scale: int!("Scale", 1),
                ty,
                max_nominal_size: int!("MaxSize", nominal_size),
                min_nominal_size: int!("MinSize", nominal_size),
                threshold: int!("Threshold", 2),
            },
        );
    }
    Ok(Some(theme))
}

static ICON_BASE_DIRS: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let mut dirs = vec![];
    dirs.push("$HOME/.icons".to_string());
    if let Ok(data_home) = var("XDG_DATA_HOME") {
        dirs.push(format!("{data_home}/icons"));
    } else {
        dirs.push("$HOME/.local/share/icons".to_string());
    }
    dirs.push("/usr/share/pixmaps".to_string());
    if let Ok(data_dirs) = var("XDG_DATA_DIRS") {
        for dir in data_dirs.split(":") {
            dirs.push(format!("{dir}/icons"));
        }
    } else {
        dirs.push("/usr/local/share/icons".to_string());
        dirs.push("/usr/share/icons".to_string());
    }
    dirs.into_iter()
        .flat_map(|d| shellexpand::full(&d).ok().map(|s| s.into_owned()))
        .map(PathBuf::from)
        .collect()
});

fn create_shm_buf(
    g: &Globals,
    data: &[u8],
    buffer_size: [i32; 2],
) -> Result<Rc<WlBuffer>, io::Error> {
    let mut memfd = uapi::memfd_create("", c::MFD_CLOEXEC | c::MFD_ALLOW_SEALING)?;
    uapi::fcntl_add_seals(memfd.raw(), c::F_SEAL_SHRINK)?;
    memfd.write_all(data)?;
    let pool = g
        .wl_shm
        .new_send_create_pool(&Rc::new(memfd.into()), data.len() as _);
    let buffer = pool.new_send_create_buffer(
        0,
        buffer_size[0],
        buffer_size[1],
        buffer_size[0] * 4,
        WlShmFormat::ARGB8888,
    );
    buffer.set_forward_to_client(false);
    pool.send_destroy();
    Ok(buffer)
}
