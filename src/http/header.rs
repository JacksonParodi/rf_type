use crate::misc::constant;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser::SerializeStruct};
use std::env;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpHeader {
    ContentTypeJson,
    ApiKey,
}

impl HttpHeader {
    pub fn as_tuple(&self) -> Result<(String, String), std::env::VarError> {
        match self {
            HttpHeader::ContentTypeJson => {
                Ok(("Content-Type".to_string(), "application/json".to_string()))
            }
            HttpHeader::ApiKey => {
                let api_key = env::var("JPCOM_API_KEY")?;
                Ok((constant::X_API_HEADER.to_string(), api_key))
            }
        }
    }
}

impl Serialize for HttpHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            HttpHeader::ContentTypeJson => {
                let mut state = serializer.serialize_struct("HttpHeader", 2)?;
                state.serialize_field("name", "Content-Type")?;
                state.serialize_field("value", "application/json")?;
                state.end()
            }
            HttpHeader::ApiKey => {
                let api_key = env::var("JPCOM_API_KEY").map_err(|_| {
                    serde::ser::Error::custom("JPCOM_API_KEY environment variable not set")
                })?;
                let mut state = serializer.serialize_struct("HttpHeader", 2)?;
                state.serialize_field("name", constant::X_API_HEADER)?;
                state.serialize_field("value", &api_key)?;
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
            "X-API-Key" => Ok(HttpHeader::ApiKey),
            _ => Err(de::Error::custom("Unknown header")),
        }
    }
}
