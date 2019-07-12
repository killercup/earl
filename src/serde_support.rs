use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use crate::Url;

/// This implementation is only available if the `serde` Cargo feature is
/// enabled.
impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// This implementation is only available if the `serde1` Cargo feature is
/// enabled.
impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UrlVisitor;

        impl<'de> Visitor<'de> for UrlVisitor {
            type Value = Url;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a URL string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                s.parse().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(UrlVisitor)
    }
}

#[test]
fn test_ser_de_url() {
    let url = Url::parse("http://www.test.com/foo/bar?$param=bazz").unwrap();
    let s = serde_json::to_string(&url).unwrap();
    let new_url: Url = serde_json::from_str(&s).unwrap();
    assert_eq!(url, new_url);
}
