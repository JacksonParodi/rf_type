use serde::{Deserialize, Serialize};
use tracing::warn;
use twitch_api::{
    eventsub::channel::chat::notification::{CommunitySubGift, Resubscription, Subscription},
    types::SubscriptionTier,
};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub enum TwitchSubscriptionTier {
    Prime,
    Tier1,
    Tier2,
    Tier3,
}

impl Ord for TwitchSubscriptionTier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use TwitchSubscriptionTier::*;

        let self_priority = match self {
            Prime => 0,
            Tier1 => 1,
            Tier2 => 2,
            Tier3 => 3,
        };

        let other_priority = match other {
            Prime => 0,
            Tier1 => 1,
            Tier2 => 2,
            Tier3 => 3,
        };

        self_priority.cmp(&other_priority)
    }
}

impl PartialOrd for TwitchSubscriptionTier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum SecretAlertTrigger {
    Bits(u32),
    Donation(u64), // in USD cents
    GiftSubscription(u32),
    Subscription(TwitchSubscriptionTier),
    Raid,
}

impl Ord for SecretAlertTrigger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use SecretAlertTrigger::*;

        let self_priority = match self {
            Bits(_) => 0,
            Donation(_) => 1,
            Subscription(_) => 2,
            GiftSubscription(_) => 3,
            Raid => 4,
        };

        let other_priority = match other {
            Bits(_) => 0,
            Donation(_) => 1,
            Subscription(_) => 2,
            GiftSubscription(_) => 3,
            Raid => 4,
        };

        match self_priority.cmp(&other_priority) {
            std::cmp::Ordering::Equal => match (self, other) {
                (Bits(a), Bits(b)) => a.cmp(b),
                (Donation(a), Donation(b)) => a.cmp(b),
                (GiftSubscription(a), GiftSubscription(b)) => a.cmp(b),
                (Subscription(a), Subscription(b)) => a.cmp(b),
                _ => std::cmp::Ordering::Equal,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for SecretAlertTrigger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Subscription> for TwitchSubscriptionTier {
    fn from(sub: Subscription) -> Self {
        get_twitch_subscription_tier(&sub.sub_tier, sub.is_prime)
    }
}

impl From<Resubscription> for TwitchSubscriptionTier {
    fn from(sub: Resubscription) -> Self {
        get_twitch_subscription_tier(&sub.sub_tier, sub.is_prime)
    }
}

impl From<CommunitySubGift> for SecretAlertTrigger {
    fn from(sub: CommunitySubGift) -> Self {
        let total = sub.total.max(1) as u32;
        SecretAlertTrigger::GiftSubscription(total)
    }
}

fn get_twitch_subscription_tier(tier: &SubscriptionTier, is_prime: bool) -> TwitchSubscriptionTier {
    match tier {
        SubscriptionTier::Tier1 => TwitchSubscriptionTier::Tier1,
        SubscriptionTier::Tier2 => TwitchSubscriptionTier::Tier2,
        SubscriptionTier::Tier3 => TwitchSubscriptionTier::Tier3,
        _ => match is_prime {
            true => TwitchSubscriptionTier::Prime,
            false => {
                warn!("unknown subscription tier, defaulting to Tier1");
                TwitchSubscriptionTier::Tier1
            }
        },
    }
}
