use crate::{
    http::{
        HttpHeader, HttpMethod, RequestPayload,
        endpoint::{
            flintstone::{FlintstoneRequestOptions, FlintstoneResponsePayload},
            log_donations::{DonationsLogRequestOptions, LogDonationsResponsePayload},
            process_new_donations::ProcessNewDonationsResponsePayload,
            random_oblique::RandomObliqueStratResponsePayload,
        },
    },
    misc::constant,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;
use url::Url;

use super::{EndpointUrl, ResponsePayload};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiRequest {
    Flintstone(FlintstoneRequestOptions),
    LogDonations(DonationsLogRequestOptions),
    // Markov(MarkovRequestOptions),
    ProcessNewDonations,
    RandomObliqueStrat,
}

impl ApiRequest {
    pub fn parse_response_payload(&self, data_value: Value) -> ResponsePayload {
        match self {
            ApiRequest::Flintstone(_) => {
                let payload: FlintstoneResponsePayload = match serde_json::from_value(data_value) {
                    Ok(payload) => payload,
                    Err(e) => {
                        error!("FlintstoneResponsePayload: Invalid value type: {:?}", e);
                        return ResponsePayload::Flintstone(FlintstoneResponsePayload::default());
                    }
                };
                ResponsePayload::Flintstone(payload)
            }
            ApiRequest::LogDonations(_) => {
                let payload: LogDonationsResponsePayload = match serde_json::from_value(data_value)
                {
                    Ok(payload) => payload,
                    Err(e) => {
                        error!("LogDonationsResponsePayload: Invalid value type: {:?}", e);
                        return ResponsePayload::LogDonations(
                            LogDonationsResponsePayload::default(),
                        );
                    }
                };
                ResponsePayload::LogDonations(payload)
            }
            ApiRequest::ProcessNewDonations => {
                let payload: ProcessNewDonationsResponsePayload =
                    match serde_json::from_value(data_value) {
                        Ok(payload) => payload,
                        Err(e) => {
                            error!(
                                "ProcessNewDonationsResponsePayload: Invalid value type: {:?}",
                                e
                            );
                            return ResponsePayload::ProcessNewDonations(
                                ProcessNewDonationsResponsePayload::default(),
                            );
                        }
                    };
                ResponsePayload::ProcessNewDonations(payload)
            }
            ApiRequest::RandomObliqueStrat => {
                let payload: RandomObliqueStratResponsePayload =
                    match serde_json::from_value(data_value) {
                        Ok(payload) => payload,
                        Err(e) => {
                            error!(
                                "RandomObliqueStratResponsePayload: Invalid value type: {:?}",
                                e
                            );
                            return ResponsePayload::RandomObliqueStrat(
                                RandomObliqueStratResponsePayload::default(),
                            );
                        }
                    };
                ResponsePayload::RandomObliqueStrat(payload)
            }
        }
    }
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
            ApiRequest::ProcessNewDonations => HttpRequest::new(
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
