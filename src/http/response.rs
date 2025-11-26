use crate::http::endpoint::{
    donation::DonationResponsePayload, flintstone::FlintstoneResponsePayload,
    random_oblique::RandomObliqueStratResponsePayload,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponsePayload {
    Donation(DonationResponsePayload),
    Flintstone(FlintstoneResponsePayload),
    RandomObliqueStrat(RandomObliqueStratResponsePayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub data: Option<ResponsePayload>,
}
