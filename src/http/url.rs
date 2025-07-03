use crate::misc::constant;
use serde::{Deserialize, Serialize};
use tracing;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EndpointUrl {
    Donation,
    Flintstone,
    // LogDonations,
    Markov,
    // ProcessNewDonations,
    RandomObliqueStrat,
}

impl EndpointUrl {
    pub fn as_url(&self) -> Url {
        let base = match Url::parse(constant::JPCOM_API_BASE_URL) {
            Ok(url) => url,
            Err(_) => {
                tracing::error!("Failed to parse base URL: {}", constant::JPCOM_API_BASE_URL);
                panic!("invalid base URL")
            }
        };

        let joined_url = match self {
            EndpointUrl::Donation => base.join(constant::DONATION_BASE_ENDPOINT),
            EndpointUrl::Flintstone => base.join(constant::FLINTSTONE_BASE_ENDPOINT),
            // EndpointUrl::LogDonations => base.join(constant::DONATIONS_LOG_BASE_ENDPOINT),
            EndpointUrl::Markov => base.join(constant::MARKOV_ENDPOINT),
            // EndpointUrl::ProcessNewDonations => base.join(constant::DONATIONS_PROCESS_NEW_ENDPOINT),
            EndpointUrl::RandomObliqueStrat => base.join(constant::RANDOM_OBLIQUE_STRAT_ENDPOINT),
        };

        match joined_url {
            Ok(url) => url,
            Err(_) => {
                tracing::error!("failed to join URL: {:?}", joined_url);
                panic!("invalid joined URL")
            }
        }
    }
}
