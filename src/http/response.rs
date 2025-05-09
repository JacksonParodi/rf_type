use crate::http::endpoint::{
    flintstone::FlintstoneResponsePayload,
    //markov::MarkovResponsePayload,
    process_new_donations::ProcessNewDonationsResponsePayload,
    random_oblique::RandomObliqueStratResponsePayload,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponsePayload {
    None,
    Flintstone(FlintstoneResponsePayload),
    // Markov(MarkovResponsePayload),
    ProcessNewDonations(ProcessNewDonationsResponsePayload),
    RandomObliqueStrat(RandomObliqueStratResponsePayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub data: Option<ResponsePayload>,
}
