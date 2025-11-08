use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum TwitchSubscriptionTier {
    Prime,
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SecretAlertTrigger {
    Bits(u32),
    Donation(u64),
    GiftSubscription(u32),
    Subscription(TwitchSubscriptionTier),
    Raid,
}
