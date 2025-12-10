pub(crate) const XRD: &str = "XDG_RUNTIME_DIR";

pub(crate) fn xrd() -> Option<String> {
    std::env::var(XRD).ok()
}
