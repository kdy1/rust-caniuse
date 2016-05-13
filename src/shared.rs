use phf;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hasher;
use std::str::FromStr;
use serde::{self, Deserialize, Deserializer};

///
/// multiple idents are required because concat_idents!() does not work.
/// Issue: https://github.com/rust-lang/rust/issues/29599
macro_rules! caniuse_enum {
    ($name:ident, $map:ident, $visitor:ident, {
        $(
            $exp:expr => $v:ident,
        )+
    }) => {
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, )]
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
                    $($name::$v => write!(f, "{}", stringify!($v)) ,)+
                }
            }
        }


        /// This is required for phf_codegen.
        /// (is wrapper type better?)
		impl Debug for $name {
			#[inline]
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				 match *self {
                    $($name::$v => write!(f, "{}::{}", stringify!($name), stringify!($v)) ,)+
                }
			}
		}

        impl $name {
        pub fn orig(&self) -> &'static str {
			match *self {
                    $($name::$v => $exp ,)+
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
            fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
             where E: serde::de::Error {
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

		impl phf::PhfHash for $name {

			#[inline]
			fn phf_hash<H: Hasher>(&self, state: &mut H) {
				state.write_u8(*self as u8);
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
    "ios_saf" => IOSSafari,
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


///
/// https://github.com/Fyrd/caniuse/blob/master/CONTRIBUTING.md#supported-changes
pub enum Support {
    /// y - (Y)es, supported by default
    Supported,
    /// a - (A)lmost supported (aka Partial support)
    Partial,
    /// n - (N)o support, or disabled by default
    No,
    /// p - No support, but has (P)olyfill
    Polyfill,
    /// u - Support (u)nknown
    Unknown,
    /// x - Requires prefi(x) to work
    PrefixRequired,
    /// d - (D)isabled by default (need to enable flag or something)
    Disabled,
}
