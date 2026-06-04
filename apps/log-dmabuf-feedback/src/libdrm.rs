use {
    bstr::ByteSlice,
    c::c_char,
    std::{
        ffi::{CStr, c_void},
        fmt::{Display, Formatter},
    },
    uapi::{
        c::{self, dev_t},
        format_ustr, opendir, readdir,
    },
};

#[link(name = "drm")]
unsafe extern "C" {
    safe fn drmGetFormatModifierVendor(modifier: u64) -> *mut c_char;
    safe fn drmGetFormatModifierName(modifier: u64) -> *mut c_char;
}

pub fn get_modifier_vendor(modifier: u64) -> Option<ForeignCStr> {
    ForeignCStr::new(drmGetFormatModifierVendor(modifier))
}

pub fn get_modifier_name(modifier: u64) -> Option<ForeignCStr> {
    ForeignCStr::new(drmGetFormatModifierName(modifier))
}

pub struct ForeignCStr {
    ptr: *mut [u8],
}

impl Drop for ForeignCStr {
    fn drop(&mut self) {
        unsafe {
            c::free(self.ptr as *mut c_void);
        }
    }
}

impl ForeignCStr {
    fn new(ptr: *mut c_char) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        Some(Self {
            ptr: unsafe { (CStr::from_ptr(ptr).to_bytes() as *const [u8]).cast_mut() },
        })
    }
}

impl Display for ForeignCStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { Display::fmt((&*self.ptr).as_bstr(), f) }
    }
}

pub fn get_nodes(dev: dev_t) -> Vec<String> {
    let major = uapi::major(dev);
    let minor = uapi::minor(dev);
    let path = format_ustr!("/sys/dev/char/{major}:{minor}/device/drm");
    let mut ret = vec![];
    let Ok(mut dir) = opendir(path) else {
        return ret;
    };
    while let Some(entry) = readdir(&mut dir) {
        let Ok(entry) = entry else {
            continue;
        };
        let name = entry.name().to_bytes().as_bstr();
        if name.starts_with(b"card") || name.starts_with(b"renderD") {
            ret.push(name.to_string());
        }
    }
    ret.sort();
    ret
}
