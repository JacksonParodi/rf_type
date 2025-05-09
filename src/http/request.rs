use crate::{
    http::{
        HttpHeader, HttpMethod, RequestPayload,
        endpoint::{
            flintstone::FlintstoneRequestOptions, log_donations::DonationsLogRequestOptions,
        },
    },
    misc::constant,
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::EndpointUrl;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiRequest {
    LogDonations(DonationsLogRequestOptions),
    Flintstone(FlintstoneRequestOptions),
    // Markov(MarkovRequestOptions),
    ProcessNewDonation,
    RandomObliqueStrat,
}

impl From<ApiRequest> for HttpRequest {
    fn from(request: ApiRequest) -> Self {
        match request {
            ApiRequest::LogDonations(options) => {
                let mut url = EndpointUrl::LogDonations.as_url();
                match options {
                    DonationsLogRequestOptions::OldDonations => {
                        url.set_query(Some(&format!("file={}", constant::OLD_DONATION_URI)));
                    }
                    DonationsLogRequestOptions::NewDonations => {
                        url.set_query(Some(&format!("file={}", constant::NEW_DONATION_URI)));
                    }
                }

                HttpRequest::new(
                    HttpMethod::GET,
                    url,
                    vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
                    None,
                )
            }
            ApiRequest::Flintstone(options) => {
                let mut url = EndpointUrl::Flintstone.as_url();
                if let FlintstoneRequestOptions::IncrementCount = options {
                    url.set_query(Some("increment=true"));
                }

                HttpRequest::new(
                    HttpMethod::GET,
                    url,
                    vec![HttpHeader::ContentTypeJson],
                    None,
                )
            }
            // ApiRequest::Markov(options) => options.into(),
            ApiRequest::ProcessNewDonation => HttpRequest::new(
                HttpMethod::GET,
                EndpointUrl::ProcessNewDonations.as_url(),
                vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
                None,
            ),
            ApiRequest::RandomObliqueStrat => HttpRequest::new(
                HttpMethod::GET,
                EndpointUrl::RandomObliqueStrat.as_url(),
                vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
                None,
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: Url,
    pub headers: Vec<HttpHeader>,
    pub body: Option<RequestPayload>,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod,
        url: Url,
        headers: Vec<HttpHeader>,
        body: Option<RequestPayload>,
    ) -> Self {
        HttpRequest {
            method,
            url,
            headers,
            body,
        }
    }
}
