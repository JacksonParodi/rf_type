pub const DONATIONS_LOG_BASE_ENDPOINT: &str = "log_donations.php";
pub const DONATIONS_PROCESS_NEW_ENDPOINT: &str = "process_new_donations.php";
pub const FLINTSTONE_BASE_ENDPOINT: &str = "flintstone.php";
pub const MARKOV_ENDPOINT: &str = "todo!";
pub const RANDOM_OBLIQUE_STRAT_ENDPOINT: &str = "random_oblique_strat.php";

pub const X_API_HEADER: &str = "X-Api-Key";
pub const NEW_DONATION_URI: &str = "new_donation";
pub const OLD_DONATION_URI: &str = "old_donation";
pub const JPCOM_API_BASE_URL: &str = "https://www.jacksonparodi.com/php/api/";

pub const FLINTSTONES_DEFAULT_COUNT: u64 = 455;
pub const OBLIQUE_STRAT_DEFAULT: &str = "Use an default colour";

pub const OVERLAY_SERVER_PORT: u16 = 8085;
pub const LOCALHOST_URL: &str = "127.0.0.1";

pub const ROLE_HEADER_BYTES: [u8; 3] = [0x00, 0x52, 0x4C];
pub const ROLE_OVERLAY_BYTES: [u8; 2] = [0x00, 0x01];
pub const ROLE_TWITCH_BOT_BYTES: [u8; 2] = [0x01, 0x01];
pub const ROLE_DISCORD_BOT_BYTES: [u8; 2] = [0x01, 0x02];
