use crate::http::{
    EndpointUrl, HttpHeader, HttpMethod, RequestPayload, ResponsePayload,
    endpoint::{
        donation::{DonationRequestAction, DonationRequestOptions, DonationResponsePayload},
        flintstone::{FlintstoneRequestOptions, FlintstoneResponsePayload},
        markov::{MarkovRequestParams, MarkovResponsePayload},
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
    // LogDonations(DonationsLogRequestOptions),
    Markov(MarkovRequestParams),
    // ProcessNewDonations,
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
            // ApiRequest::LogDonations(_) => {
            //     ResponsePayload::LogDonations(LogDonationsResponsePayload::from(data_value))
            // }
            ApiRequest::Markov(_) => {
                ResponsePayload::Markov(MarkovResponsePayload::from(data_value))
            }
            // ApiRequest::ProcessNewDonations => ResponsePayload::ProcessNewDonations(
            //     ProcessNewDonationsResponsePayload::from(data_value),
            // ),
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
                    DonationRequestAction::Process => HttpMethod::POST,
                    DonationRequestAction::Add => HttpMethod::POST,
                };

                let mut url = EndpointUrl::Donation.as_url();
                url.set_query(Some(&format!("action={}", options.action.to_string())));

                match options.action {
                    DonationRequestAction::Fetch => {
                        if let Some(file) = options.file {
                            url.set_query(Some(&format!("file={}", file.to_string())));
                        }
                        // download not currently supported
                    }
                    DonationRequestAction::Process => {}
                    DonationRequestAction::Add => {
                        // adding donation not currently supported
                    }
                }

                HttpRequest::new(
                    method,
                    url,
                    vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
                    None,
                )
            }
            // ApiRequest::LogDonations(options) => {
            //     let mut url = EndpointUrl::LogDonations.as_url();
            //     match options {
            //         DonationsLogRequestOptions::OldDonations => {
            //             url.set_query(Some(&format!("file={}", constant::OLD_DONATION_URI)));
            //         }
            //         DonationsLogRequestOptions::NewDonations => {
            //             url.set_query(Some(&format!("file={}", constant::NEW_DONATION_URI)));
            //         }
            //     }

            //     HttpRequest::new(
            //         HttpMethod::GET,
            //         url,
            //         vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
            //         None,
            //     )
            // }
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
            ApiRequest::Markov(options) => {
                let mut url = EndpointUrl::Markov.as_url();

                if let Some(seed) = options.seed {
                    url.set_query(Some(&format!("seed={}", seed)));
                }

                HttpRequest::new(
                    HttpMethod::GET,
                    url,
                    vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
                    None,
                )
            }
            // ApiRequest::ProcessNewDonations => HttpRequest::new(
            //     HttpMethod::GET,
            //     EndpointUrl::ProcessNewDonations.as_url(),
            //     vec![HttpHeader::ContentTypeJson, HttpHeader::ApiKey],
            //     None,
            // ),
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
