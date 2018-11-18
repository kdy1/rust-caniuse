pub use self::support::*;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

mod support;

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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name {
            $(
                #[doc = "\""]
                #[doc = $exp]
                #[doc = "\""]
                #[serde(rename = $exp)]
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

        impl $name {
           pub fn as_str(&self) -> &'static str {
	    		match *self {
                    $($name::$v => $exp ,)+
                }
            }
        }

        impl FromStr for $name {
            type Err = ();
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {

                match s {
                    $($exp => Ok($name::$v),)*
                    _ => Err(()),
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
    "ios_saf" => IOSSafari,
    "op_mini" => OperaMini,
    "android" => AndroidBrowser,
    "bb" => BlackberryBrowser,
    "op_mob" => OperaMobile,
    "and_chr" => AndroidChrome,
    "and_ff" => AndroidFirefox,
    "ie_mob" => IEMobile,
    "baidu" => BaiduBrowser,
    "and_uc" => AndroidUCBrowser,
    "and_qq" => AndroidQQBrowser,
    "samsung" => SamsungBrowser,
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
