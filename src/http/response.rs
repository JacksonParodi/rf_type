use crate::http::endpoint::{
    flintstone::FlintstoneResponsePayload,
    //markov::MarkovResponsePayload,
    process_new_donations::ProcessNewDonationsResponsePayload,
    random_oblique::GetRandomObliqueStratResponsePayload,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponsePayload {
    None,
    Flintstone(FlintstoneResponsePayload),
    // Markov(MarkovResponsePayload),
    ProcessNewDonation(ProcessNewDonationsResponsePayload),
    RandomObliqueStrat(GetRandomObliqueStratResponsePayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub status: String,
    pub message: String,
    pub data: Option<ResponsePayload>,
}
