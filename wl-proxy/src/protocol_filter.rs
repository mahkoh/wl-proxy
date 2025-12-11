mod prototyping;

use linearize::StaticCopyMap;
use crate::protocols::ProxyInterface;

pub struct ProtocolFilter {
    baseline: &'static StaticCopyMap<ProxyInterface, u32>,
}

impl ProtocolFilter {
    pub fn baseline_i_am_prototyping() -> Self {
        Self { baseline: prototyping::BASELINE }
    }
}
