//! enums are predeclared to verify them on compile time.

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{self, Deserialize, Deserializer};
use phf::{self};

///
/// multiple idents are required because concat_idents!() does not work.
/// Issue: https://github.com/rust-lang/rust/issues/29599
macro_rules! caniuse_enum {
    ($name:ident, $map:ident, $visitor:ident, {
        $(
            $exp:expr => $v:ident,
        )+
    }) => {
        #[repr(C)]
        #[repr(u16)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                #[doc = "\""]
                #[doc = $exp]
                #[doc = "\""]
                $v,
            )+
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match *self {
                    $($name::$v => f.write_str(stringify!($v)),)+
                }
            }
        }

        pub static $map: phf::Map<&'static str, $name> = phf_map! {
            $($exp => $name::$v,)+
        };

        impl FromStr for $name {
            type Err = ();
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match $map.get(s) {
                    Some(o) => { Ok(*o) },
                    None => { Err(()) }
                }
            }
        }


        impl Deserialize for $name {
            fn deserialize<D: Deserializer>(d: &mut D)
             -> Result<Self, D::Error> {
                d.deserialize($visitor)
            }
        }

        struct $visitor;

        impl serde::de::Visitor for $visitor {
            type Value = $name;

            #[inline]
            fn visit_str<E>(&mut self, v: &str)
            -> Result<Self::Value, E> where E: serde::de::Error {
                match $map.get(v) {
                    Some(o) => { Ok(*o) },
                    None => {
                        Err(E::invalid_value(
                            &format!("Unknown value('{}') for {}", v, stringify!($name))
                        ))
                    },
                }
            }
        }
    }
}



caniuse_enum!(Browser, BROWSERS, BrowserVisitor, {
    "ie" => IE,
    "edge" => Edge,
    "firefox" => Firefox,
    "chrome" => Chrome,
    "safari" => Safari,
    "opera" => Opera,
    "ios_saf" => iOSSafari,
    "op_mini" => OperaMini,
    "android" => AndroidBrowser,
    "bb" => BlackberryBrowser,
    "op_mob" => OperaMobile,
    "and_chr" => AndroidChrome,
    "and_ff" => AndroidFirefox,
    "ie_mob" => IEMobile,
    "and_uc" => AndroidUCBrowser,
});

caniuse_enum!(Status, STATUSES, StatusVisitor, {
    "rec" => Recommendation,
    "cr" => CandidateRecommendation,
    "ls" => LivingStandard,
    "pr" => ProposedRecommendation,
    "wd" => WorkingDraft,
    "other" => Other,
    "unoff" => Unofficial,
});

/// Vendor prefixes.
caniuse_enum!(Prefix, PREFIXES, PrefixVisitor, {
    "ms" => Ms,
    "moz" => Moz,
    "webkit" => Webkit,
    "o" => Opera,
});
