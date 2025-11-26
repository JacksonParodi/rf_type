use crate::http::{
    EndpointUrl, HttpHeader, HttpMethod, RequestPayload, ResponsePayload,
    endpoint::{
        donation::{DonationRequestAction, DonationRequestOptions, DonationResponsePayload},
        flintstone::{FlintstoneRequestOptions, FlintstoneResponsePayload},
        random_oblique::RandomObliqueStratResponsePayload,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiRequest {
    Donation(DonationRequestOptions),
    Flintstone(FlintstoneRequestOptions),
    RandomObliqueStrat,
}

impl ApiRequest {
    pub fn parse_response_payload(&self, data_value: Value) -> ResponsePayload {
        match self {
            ApiRequest::Donation(_) => {
                ResponsePayload::Donation(DonationResponsePayload::from(data_value))
            }
            ApiRequest::Flintstone(_) => {
                ResponsePayload::Flintstone(FlintstoneResponsePayload::from(data_value))
            }
            ApiRequest::RandomObliqueStrat => ResponsePayload::RandomObliqueStrat(
                RandomObliqueStratResponsePayload::from(data_value),
            ),
        }
    }
}

impl From<ApiRequest> for HttpRequest {
    fn from(request: ApiRequest) -> Self {
        match request {
            ApiRequest::Donation(options) => {
                let method = match options.action {
                    DonationRequestAction::Fetch => HttpMethod::GET,
                    DonationRequestAction::Add => HttpMethod::POST,
                };

                let mut url = EndpointUrl::Donation.as_url();
                url.query_pairs_mut()
                    .append_pair("action", &options.action.to_string());

                match options.action {
                    DonationRequestAction::Fetch => {
                        if let Some(amount) = options.amount {
                            url.query_pairs_mut()
                                .append_pair("amount", &amount.to_string());
                        }
                    }
                    DonationRequestAction::Add => {
                        // adding donation not currently supported
                    }
                }

                HttpRequest::new(
                    method,
                    url,
                    vec![HttpHeader::ContentTypeJson, HttpHeader::ApiToken],
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
                    vec![HttpHeader::ContentTypeJson, HttpHeader::ApiToken],
                    None,
                )
            }
            ApiRequest::RandomObliqueStrat => HttpRequest::new(
                HttpMethod::GET,
                EndpointUrl::RandomObliqueStrat.as_url(),
                vec![HttpHeader::ContentTypeJson, HttpHeader::ApiToken],
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
