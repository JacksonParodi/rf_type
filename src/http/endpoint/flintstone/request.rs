use crate::http::{HttpHeader, HttpMethod, HttpRequest, url::EndpointUrl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FlintstoneRequest {
    GetCurrentCount,
    IncrementCount,
}

impl FlintstoneRequest {
    pub fn to_http_request(&self) -> HttpRequest {
        match self {
            FlintstoneRequest::GetCurrentCount => HttpRequest::new(
                HttpMethod::GET,
                EndpointUrl::Flintstone.as_url(),
                vec![HttpHeader::ContentTypeJson],
                None,
            ),
            FlintstoneRequest::IncrementCount => {
                let mut flintstone_url = EndpointUrl::Flintstone.as_url();
                flintstone_url.set_query(Some("increment=true"));

                HttpRequest::new(
                    HttpMethod::GET,
                    flintstone_url,
                    vec![HttpHeader::ContentTypeJson],
                    None,
                )
            }
        }
    }
}
