use crate::http::endpoint::{
    flintstone::FlintstoneResponsePayload,
    //markov::MarkovResponsePayload,
    process_new_donations::ProcessNewDonationsResponsePayload,
    random_oblique::RandomObliqueStratResponsePayload,
};
use serde::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponsePayload {
    None,
    Flintstone(FlintstoneResponsePayload),
    // Markov(MarkovResponsePayload),
    ProcessNewDonations(ProcessNewDonationsResponsePayload),
    RandomObliqueStrat(RandomObliqueStratResponsePayload),
}

// impl From<Value> for ResponsePayload {
//     fn from(value: Value) -> Self {
//         match serde_json::from_value(value.clone()) {
//             Ok(payload) => payload,
//             Err(e) => {
//                 tracing::error!(
//                     "{:?} ResponsePayload: Invalid value type: {:?}",
//                     e,
//                     value.clone()
//                 );
//                 ResponsePayload::None
//             }
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub data: Option<ResponsePayload>,
}
