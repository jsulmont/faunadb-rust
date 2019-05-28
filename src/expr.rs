mod object;
mod reference;
mod set;

use base64;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{
    ser::Serializer,
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};
use serde_json::Value;
use std::borrow::Cow;

pub use object::Object;
pub use reference::Ref;
pub use set::Set;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    String(Cow<'a, str>),
    Double(f64),
    Float(f32),
    Int(i64),
    UInt(u64),
    Boolean(bool),
    Null,
    Object(Object<'a>),
    Bytes(Cow<'a, [u8]>),
    Date(NaiveDate),
    Ref(Ref<'a>),
    Array(Vec<Expr<'a>>),
    Set(Box<Set<'a>>),
    Timestamp(DateTime<Utc>),
}

impl<'a> From<Value> for Expr<'a> {
    fn from(val: Value) -> Self {
        match val {
            Value::Null => Expr::Null,
            Value::Bool(b) => Expr::from(b),
            Value::Number(num) => {
                if num.is_i64() {
                    Expr::from(num.as_i64().unwrap())
                } else if num.is_u64() {
                    Expr::from(num.as_u64().unwrap())
                } else {
                    Expr::from(num.as_f64().unwrap())
                }
            }
            Value::String(s) => Expr::from(s),
            _ => unimplemented!(),
        }
    }
}

impl<'a> Serialize for Expr<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Expr::String(s) => serializer.serialize_str(&*s),
            Expr::Double(d) => serializer.serialize_f64(*d),
            Expr::Float(f) => serializer.serialize_f32(*f),
            Expr::Int(i) => serializer.serialize_i64(*i),
            Expr::UInt(i) => serializer.serialize_u64(*i),
            Expr::Boolean(b) => serializer.serialize_bool(*b),
            Expr::Null => serializer.serialize_none(),
            Expr::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                map.serialize_entry("object", &obj)?;
                map.end()
            }
            Expr::Bytes(b) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("@bytes", &base64::encode(b))?;
                map.end()
            }
            Expr::Date(d) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("@date", &d.format("%Y-%m-%d").to_string())?;
                map.end()
            }
            Expr::Ref(r) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("@ref", &r.path())?;
                map.end()
            }
            Expr::Array(ary) => {
                let mut seq = serializer.serialize_seq(Some(ary.len()))?;
                for element in ary {
                    seq.serialize_element(&element)?;
                }
                seq.end()
            }
            Expr::Set(s) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("@set", &s)?;
                map.end()
            }
            Expr::Timestamp(dt) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("@ts", &dt.to_rfc3339())?;
                map.end()
            }
        }
    }
}

impl<'a> From<&'a str> for Expr<'a> {
    fn from(s: &'a str) -> Expr<'a> {
        Expr::String(Cow::from(s))
    }
}

impl<'a> From<String> for Expr<'a> {
    fn from(s: String) -> Expr<'a> {
        Expr::String(Cow::from(s))
    }
}

impl<'a> From<f64> for Expr<'a> {
    fn from(f: f64) -> Expr<'a> {
        Expr::Double(f)
    }
}

impl<'a> From<f32> for Expr<'a> {
    fn from(f: f32) -> Expr<'a> {
        Expr::Float(f)
    }
}

impl<'a> From<i8> for Expr<'a> {
    fn from(i: i8) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<i16> for Expr<'a> {
    fn from(i: i16) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<i32> for Expr<'a> {
    fn from(i: i32) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<i64> for Expr<'a> {
    fn from(i: i64) -> Expr<'a> {
        Expr::Int(i)
    }
}

impl<'a> From<isize> for Expr<'a> {
    fn from(i: isize) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<u8> for Expr<'a> {
    fn from(u: u8) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<u16> for Expr<'a> {
    fn from(u: u16) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<u32> for Expr<'a> {
    fn from(u: u32) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<u64> for Expr<'a> {
    fn from(u: u64) -> Expr<'a> {
        Expr::UInt(u)
    }
}

impl<'a> From<usize> for Expr<'a> {
    fn from(u: usize) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<bool> for Expr<'a> {
    fn from(b: bool) -> Expr<'a> {
        Expr::Boolean(b)
    }
}

impl<'a, O> From<O> for Expr<'a>
where
    O: Into<Object<'a>>,
{
    fn from(o: O) -> Expr<'a> {
        Expr::Object(o.into())
    }
}

impl<'a> From<Vec<u8>> for Expr<'a> {
    fn from(b: Vec<u8>) -> Expr<'a> {
        Expr::Bytes(Cow::from(b))
    }
}

impl<'a> From<&'a [u8]> for Expr<'a> {
    fn from(b: &'a [u8]) -> Expr<'a> {
        Expr::Bytes(Cow::from(b))
    }
}

impl<'a> From<Vec<Expr<'a>>> for Expr<'a> {
    fn from(a: Vec<Expr<'a>>) -> Expr<'a> {
        Expr::Array(a)
    }
}

impl<'a> From<Ref<'a>> for Expr<'a> {
    fn from(r: Ref<'a>) -> Expr<'a> {
        Expr::Ref(r)
    }
}

impl<'a> From<NaiveDate> for Expr<'a> {
    fn from(d: NaiveDate) -> Expr<'a> {
        Expr::Date(d)
    }
}

impl<'a> From<Set<'a>> for Expr<'a> {
    fn from(s: Set<'a>) -> Expr<'a> {
        Expr::Set(Box::new(s))
    }
}

impl<'a> From<DateTime<Utc>> for Expr<'a> {
    fn from(dt: DateTime<Utc>) -> Expr<'a> {
        Expr::Timestamp(dt)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{DateTime, NaiveDate, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_string_expr() {
        let expr = Expr::from("cat");
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("\"cat\"", serialized);
    }

    #[test]
    fn test_f64_expr() {
        let expr = Expr::from(4.12f64);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4.12", serialized);
    }

    #[test]
    fn test_f32_expr() {
        let expr = Expr::from(4.12f32);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4.12", serialized);
    }

    #[test]
    fn test_i8_expr() {
        let expr = Expr::from(4i8);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_i16_expr() {
        let expr = Expr::from(4i16);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_i32_expr() {
        let expr = Expr::from(4i32);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_i64_expr() {
        let expr = Expr::from(4i64);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_isize_expr() {
        let expr = Expr::from(4isize);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u8_expr() {
        let expr = Expr::from(4u8);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u16_expr() {
        let expr = Expr::from(4u16);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u32_expr() {
        let expr = Expr::from(4u32);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u64_expr() {
        let expr = Expr::from(4u64);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_usize_expr() {
        let expr = Expr::from(4usize);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_bytes_expr() {
        let bytes = vec![0x1, 0x2, 0x3, 0x4];
        let expr = Expr::from(bytes.as_slice());
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"@bytes\":\"AQIDBA==\"}", serialized)
    }

    #[test]
    fn test_date_expr() {
        let expr = Expr::from(NaiveDate::from_ymd(2001, 5, 31));
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"@date\":\"2001-05-31\"}", serialized)
    }

    #[test]
    fn test_ref_with_class_expr() {
        let expr = Expr::from(Ref::new("foo", Ref::class("test")));
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"@ref\":\"classes/test/foo\"}", serialized)
    }

    #[test]
    fn test_bool_expr() {
        let expr = Expr::from(true);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("true", serialized)
    }

    #[test]
    fn test_null_expr() {
        let expr = Expr::Null;
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("null", serialized)
    }

    #[test]
    fn test_simple_array_expr() {
        let array = vec![Expr::from(1), Expr::from("test")];
        let expr = Expr::from(array);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("[1,\"test\"]", serialized)
    }

    #[test]
    fn test_complex_array_expr() {
        let mut object = Object::new();
        object.insert("foo", "bar");
        object.insert("lol", false);

        let array = vec![Expr::from(1), Expr::from(object)];
        let expr = Expr::from(array);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!(
            "[1,{\"object\":{\"foo\":\"bar\",\"lol\":false}}]",
            serialized
        )
    }

    #[test]
    fn test_object_expr() {
        let mut object = Object::new();
        object.insert("foo", "bar");
        object.insert("lol", false);

        let expr = Expr::from(object);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"object\":{\"foo\":\"bar\",\"lol\":false}}", serialized)
    }

    #[test]
    fn test_set_expr() {
        let set = Set::matching(Ref::index("cats_age"), 8);
        let expr = Expr::from(set);
        let serialized = serde_json::to_value(&expr).unwrap();

        let expected = json!({
            "@set": {
                "match": { "@ref": "indexes/cats_age" },
                "terms": 8
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_set_timestamp_expr() {
        let dt_str = "2019-05-26T16:20:00Z";
        let dt = DateTime::parse_from_rfc3339(dt_str)
            .unwrap()
            .with_timezone(&Utc);

        let expr = Expr::from(dt);
        let serialized = serde_json::to_value(&expr).unwrap();

        let expected = json!({ "@ts": "2019-05-26T16:20:00+00:00" });

        assert_eq!(expected, serialized);
    }
}
