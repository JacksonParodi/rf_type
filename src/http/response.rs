use crate::http::endpoint::{
    donation::DonationResponsePayload, flintstone::FlintstoneResponsePayload,
    markov::MarkovResponsePayload, random_oblique::RandomObliqueStratResponsePayload,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponsePayload {
    Donation(DonationResponsePayload),
    Flintstone(FlintstoneResponsePayload),
    // LogDonations(LogDonationsResponsePayload),
    Markov(MarkovResponsePayload),
    // ProcessNewDonations(ProcessNewDonationsResponsePayload),
    RandomObliqueStrat(RandomObliqueStratResponsePayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub data: Option<ResponsePayload>,
}
