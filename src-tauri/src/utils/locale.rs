use serde::{Serialize, Deserialize, Serializer, Deserializer, ser::Error};

#[derive(Debug, Clone, Copy)]
pub enum Locale {
    English,
    Spanish,
}

impl Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            Locale::English => "en",
            Locale::Spanish => "es",
        })
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "en" => Ok(Locale::English),
            "es" => Ok(Locale::Spanish),
            _ => Err(D::Error::custom("Expected en or es")),
        }
    }
}


