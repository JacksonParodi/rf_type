use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tracing::warn;
use twitch_api::{
    eventsub::channel::chat::notification::{
        CommunitySubGift as ForeignCommunitySubGift, Resubscription as ForeignResubscription,
        Subscription as ForeignSubscription,
    },
    types::SubscriptionTier as ForeignSubscriptionTier,
};

const TIER1_SLUG: &str = "t1";
const TIER2_SLUG: &str = "t2";
const TIER3_SLUG: &str = "t3";
const PRIME_SLUG: &str = "p";
const SUB_SLUG: &str = "sub_";
const GIFT_SUB_SLUG: &str = "gift_";
const DONATION_SLUG: &str = "dono_";
const BITS_SLUG: &str = "bits_";
const RAID_SLUG: &str = "raid";

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub enum TwitchSubscriptionTier {
    Prime,
    Tier1,
    Tier2,
    Tier3,
}

impl FromStr for TwitchSubscriptionTier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PRIME_SLUG => Ok(TwitchSubscriptionTier::Prime),
            TIER1_SLUG => Ok(TwitchSubscriptionTier::Tier1),
            TIER2_SLUG => Ok(TwitchSubscriptionTier::Tier2),
            TIER3_SLUG => Ok(TwitchSubscriptionTier::Tier3),
            _ => Err(()),
        }
    }
}

impl ToString for TwitchSubscriptionTier {
    fn to_string(&self) -> String {
        match self {
            TwitchSubscriptionTier::Prime => PRIME_SLUG.to_string(),
            TwitchSubscriptionTier::Tier1 => TIER1_SLUG.to_string(),
            TwitchSubscriptionTier::Tier2 => TIER2_SLUG.to_string(),
            TwitchSubscriptionTier::Tier3 => TIER3_SLUG.to_string(),
        }
    }
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

impl ToString for SecretAlertTrigger {
    fn to_string(&self) -> String {
        match self {
            SecretAlertTrigger::Bits(amount) => format!("{}{}", BITS_SLUG, amount),
            SecretAlertTrigger::Donation(amount) => format!("{}{}", DONATION_SLUG, amount),
            SecretAlertTrigger::GiftSubscription(amount) => format!("{}{}", GIFT_SUB_SLUG, amount),
            SecretAlertTrigger::Subscription(tier) => {
                format!("{}{}", SUB_SLUG, tier.to_string())
            }
            SecretAlertTrigger::Raid => RAID_SLUG.to_string(),
        }
    }
}

impl FromStr for SecretAlertTrigger {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == RAID_SLUG {
            return Ok(SecretAlertTrigger::Raid);
        }

        if let Some(amount_str) = s.strip_prefix(BITS_SLUG) {
            if let Ok(amount) = amount_str.parse::<u32>() {
                return Ok(SecretAlertTrigger::Bits(amount));
            }
        }

        if let Some(amount_str) = s.strip_prefix(DONATION_SLUG) {
            if let Ok(amount) = amount_str.parse::<u64>() {
                return Ok(SecretAlertTrigger::Donation(amount));
            }
        }

        if let Some(count_str) = s.strip_prefix(GIFT_SUB_SLUG) {
            if let Ok(count) = count_str.parse::<u32>() {
                return Ok(SecretAlertTrigger::GiftSubscription(count));
            }
        }

        if let Some(tier_str) = s.strip_prefix(SUB_SLUG) {
            match tier_str {
                PRIME_SLUG => Ok(SecretAlertTrigger::Subscription(
                    TwitchSubscriptionTier::Prime,
                )),
                TIER1_SLUG => Ok(SecretAlertTrigger::Subscription(
                    TwitchSubscriptionTier::Tier1,
                )),
                TIER2_SLUG => Ok(SecretAlertTrigger::Subscription(
                    TwitchSubscriptionTier::Tier2,
                )),
                TIER3_SLUG => Ok(SecretAlertTrigger::Subscription(
                    TwitchSubscriptionTier::Tier3,
                )),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
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

impl From<ForeignSubscription> for TwitchSubscriptionTier {
    fn from(sub: ForeignSubscription) -> Self {
        get_twitch_subscription_tier(&sub.sub_tier, sub.is_prime)
    }
}

impl From<ForeignResubscription> for TwitchSubscriptionTier {
    fn from(sub: ForeignResubscription) -> Self {
        get_twitch_subscription_tier(&sub.sub_tier, sub.is_prime)
    }
}

impl From<ForeignCommunitySubGift> for SecretAlertTrigger {
    fn from(sub: ForeignCommunitySubGift) -> Self {
        let total = sub.total.max(1) as u32;
        SecretAlertTrigger::GiftSubscription(total)
    }
}

fn get_twitch_subscription_tier(
    tier: &ForeignSubscriptionTier,
    is_prime: bool,
) -> TwitchSubscriptionTier {
    if is_prime {
        return TwitchSubscriptionTier::Prime;
    }

    match tier {
        ForeignSubscriptionTier::Tier1 => TwitchSubscriptionTier::Tier1,
        ForeignSubscriptionTier::Tier2 => TwitchSubscriptionTier::Tier2,
        ForeignSubscriptionTier::Tier3 => TwitchSubscriptionTier::Tier3,
        _ => {
            warn!("unknown subscription tier, defaulting to Tier1");
            TwitchSubscriptionTier::Tier1
        }
    }
}
