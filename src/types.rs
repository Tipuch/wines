use diesel::pg::{Pg};
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::serialize;
use diesel::serialize::{Output, ToSql, IsNull};
use std::io::Write;
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};

#[derive(SqlType)]
#[postgres(type_name = "wine_color")]
pub struct WineColor;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "WineColor"]
pub enum WineColorEnum {
    Red,
    White,
    Pink
}

struct WineColorVisitor;

impl<'de> Visitor<'de> for WineColorVisitor {
    type Value = WineColorEnum;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a lowercase string red, white or pink.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value.to_lowercase().as_ref() {
            "red" => Ok(WineColorEnum::Red),
            "white" => Ok(WineColorEnum::White),
            "pink" => Ok(WineColorEnum::Pink),
            _ => Err(de::Error::custom(format!("invalid wine color: {}", value)))
        }
    }

}

impl<'de> Deserialize<'de> for WineColorEnum {
    fn deserialize<D>(deserializer: D) -> Result<WineColorEnum, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WineColorVisitor)
    }
}

impl Serialize for WineColorEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            WineColorEnum::Red => serializer.serialize_str("red"),
            WineColorEnum::White => serializer.serialize_str("white"),
            WineColorEnum::Pink => serializer.serialize_str("pink")
        }
    }
}

impl ToSql<WineColor, Pg> for WineColorEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            WineColorEnum::Red => out.write_all(b"red")?,
            WineColorEnum::White => out.write_all(b"white")?,
            WineColorEnum::Pink => out.write_all(b"pink")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<WineColor, Pg> for WineColorEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"red" => Ok(WineColorEnum::Red),
            b"white" => Ok(WineColorEnum::White),
            b"pink" => Ok(WineColorEnum::Pink),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}