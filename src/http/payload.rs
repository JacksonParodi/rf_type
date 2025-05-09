use crate::endpoint::{
    flintstone::FlintstoneResponsePayload, markov::MarkovResponsePayload,
    process_new_donation::ProcessNewDonationResponsePayload,
    random_oblique::GetRandomObliqueStratResponsePayload,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RfResponsePayload {
    None,
    Flintstone(FlintstoneResponsePayload),
    Markov(MarkovResponsePayload),
    ProcessNewDonation(ProcessNewDonationResponsePayload),
    RandomObliqueStrat(GetRandomObliqueStratResponsePayload),
}
