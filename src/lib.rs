#![feature(plugin,custom_derive)]
#![plugin(phf_macros,serde_macros)]

//! caniuse-rs
//!

extern crate phf;
extern crate serde;
mod shared;
pub use shared::*;


#[derive(Debug)]
pub struct Feature {
    /// ID of the feature.
    pub id: &'static str,
    pub title: &'static str,
    /// ID of the parent feature, or empty string.
    pub parent_id: &'static str,
    /// Specification status
    pub status: Status,
    pub stats: phf::Map<Browser, phf::Map<&'static str, &'static str>>,
}

impl Feature {
    pub fn parent(&'static self) -> Option<&'static Feature> {
        if self.parent_id == "" {
            return None;
        }

        match FEATURES.get(self.parent_id) {
            Some(o) => Some(o),
            None => unreachable!(),
        }
    }
}

impl std::str::FromStr for &'static Feature {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<&'static Feature, Self::Err> {
        match FEATURES.get(s) {
            Some(o) => Ok(o),
            None => Err(()),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/datas.rs"));
