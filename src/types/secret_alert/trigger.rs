use serde::{Deserialize, Serialize};
use tracing::warn;
use twitch_api::{
    eventsub::channel::chat::notification::{CommunitySubGift, Subscription},
    types::SubscriptionTier,
};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub enum TwitchSubscriptionTier {
    Prime,
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum SecretAlertTrigger {
    Bits(u32),
    Donation(u64), // in USD cents
    GiftSubscription(u32),
    Subscription(TwitchSubscriptionTier),
    Raid,
}

impl From<Subscription> for TwitchSubscriptionTier {
    fn from(sub: Subscription) -> Self {
        match &sub.sub_tier {
            SubscriptionTier::Tier1 => TwitchSubscriptionTier::Tier1,
            SubscriptionTier::Tier2 => TwitchSubscriptionTier::Tier2,
            SubscriptionTier::Tier3 => TwitchSubscriptionTier::Tier3,
            _ => match sub.is_prime {
                true => TwitchSubscriptionTier::Prime,
                false => {
                    warn!("unknown subscription tier, defaulting to Tier1");
                    TwitchSubscriptionTier::Tier1
                }
            },
        }
    }
}

impl From<CommunitySubGift> for SecretAlertTrigger {
    fn from(sub: CommunitySubGift) -> Self {
        let total = sub.total.max(1) as u32;
        SecretAlertTrigger::GiftSubscription(total)
    }
}
