//! caniuse-rs contains static database from
//! [caniuse-db by Fyrd](https://github.com/Fyrd/caniuse).
//!
//!

extern crate phf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
mod shared;
pub use shared::*;

/// version:support map
pub type Stat = phf::Map<&'static str, Support>;
/// browser:stat map
pub type Stats = [(Browser, Stat); 15];

impl Feature {
    #[inline]
    pub fn parent(self) -> Option<Self> {
        let parent_id: &'static str = self.parent_id();
        if parent_id == "" {
            return None;
        }

        match FEATURES.get(parent_id) {
            Some(o) => Some(*o),
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn stat(self, br: Browser) -> Option<&'static Stat> {
        for &(browser, ref s) in self.stats().iter() {
            if br == browser {
                return Some(s);
            }
        }
        None
    }
}

impl std::str::FromStr for Feature {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match FEATURES.get(s) {
            Some(o) => Ok(*o),
            None => Err(()),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/datas.rs"));
