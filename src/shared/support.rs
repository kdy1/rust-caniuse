use serde::{self, Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

///
/// https://github.com/Fyrd/caniuse/blob/master/CONTRIBUTING.md#supported-changes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// custom - Partial + PrefixRequired
    PrefixedPartial,
}

impl FromStr for Support {
    type Err = SupportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Support::*;
        assert!(s.len() != 0);
        assert!(s.is_ascii());

        let mut chars = s.char_indices();

        let mut ret = None;
        while let Some((i, c)) = chars.next() {
            let v = match c {
                'y' => Supported,
                'a' => Partial,
                // return
                'n' => return Ok(No),
                'p' => Polyfill,
                'u' => Unknown,
                'x' => PrefixRequired,
                // return, as it's useless anyway.
                'd' => return Ok(Disabled),
                '#' | ' ' | '0'...'9' => continue, // ignore
                _ => return Err(SupportParseError::Invalid(i)),
            };

            if ret == None {
                ret = Some(v);
            } else if ret == Some(Supported) && v == PrefixRequired {
                // 'y x'
                return Ok(PrefixRequired);
            } else if ret == Some(Partial) && v == PrefixRequired {
                // 'a x'
                return Ok(PrefixedPartial);
            } else {
                return Err(SupportParseError::Invalid(i));
            }
        }

        if let Some(v) = ret {
            Ok(v)
        } else {
            Err(SupportParseError::Invalid(0))
        }
    }
}

pub enum SupportParseError {
    /// char index
    Invalid(usize),
}

impl<'de> Deserialize<'de> for Support {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SupportVisitor)
    }
}

struct SupportVisitor;

impl<'de> serde::de::Visitor<'de> for SupportVisitor {
    type Value = Support;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "support string")
    }

    #[inline]
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Support::from_str(s).map_err(|_| E::custom(format!("invalid value for Support: '{}'", s)))
    }
}

impl Default for Support {
    fn default() -> Self {
        Support::No
    }
}
