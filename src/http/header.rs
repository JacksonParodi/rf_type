use serde::de;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq)]
pub enum HttpHeader {
    ContentTypeJson,
    ApiKey(String),
}

impl Serialize for HttpHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            HttpHeader::ContentTypeJson => {
                // "Content-Type: application/json"
                let mut state = serializer.serialize_struct("HttpHeader", 2)?;
                state.serialize_field("name", "Content-Type")?;
                state.serialize_field("value", "application/json")?;
                state.end()
            }
            HttpHeader::ApiKey(val) => {
                // "X-API-Key: <val>"
                let mut state = serializer.serialize_struct("HttpHeader", 2)?;
                state.serialize_field("name", "X-API-Key")?;
                state.serialize_field("value", val)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for HttpHeader {
    fn deserialize<D>(deserializer: D) -> Result<HttpHeader, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct HeaderHelper {
            name: String,
            value: String,
        }

        let helper = HeaderHelper::deserialize(deserializer)?;
        match helper.name.as_str() {
            "Content-Type" if helper.value == "application/json" => Ok(HttpHeader::ContentTypeJson),
            "X-API-Key" => Ok(HttpHeader::ApiKey(helper.value)),
            _ => Err(de::Error::custom("Unknown header")),
        }
    }
}
