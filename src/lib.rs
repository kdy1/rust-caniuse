//!
//! caniuse-rs
//!

#![feature(plugin, custom_derive)]
#![plugin(serde_macros, phf_macros)]


extern crate phf;
extern crate serde;

mod shared;
pub use shared::*;

#[derive(Debug)]
pub struct Feature {
    pub id:    &'static str,
    pub title: &'static str,
    pub parent: &'static str,
    pub status: Status,

    // pub stats: HashMap<Browser, HashMap<String, String>>,
}

impl std::str::FromStr for &'static Feature {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match FEATURES.get(s) {
            Some(o) => { Ok(o) },
            None => { Err(()) }
        }
    }
}



include!(concat!(env!("OUT_DIR"), "/datas.rs"));
