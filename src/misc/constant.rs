pub const DONATION_BASE_ENDPOINT: &str = "donation.php";
pub const FLINTSTONE_BASE_ENDPOINT: &str = "flintstone.php";
pub const RANDOM_OBLIQUE_STRAT_ENDPOINT: &str = "random_oblique_strat.php";

pub const X_API_HEADER: &str = "X-Api-Token";
pub const JPCOM_API_BASE_URL: &str = "https://www.jacksonparodi.com/api/";
pub const API_TOKEN_ENV_VAR: &str = "JPCOM_API_TOKEN";

pub const FLINTSTONES_DEFAULT_COUNT: u64 = 455;
pub const OBLIQUE_STRAT_DEFAULT: &str = "Use an default colour";

pub const OVERLAY_SERVER_PORT: u16 = 8085;
pub const LOCALHOST_URL: &str = "127.0.0.1";

pub const ROLE_HEADER_BYTES: [u8; 3] = [0x00, 0x52, 0x4C];
pub const ROLE_OVERLAY_BYTES: [u8; 2] = [0x00, 0x01];
pub const ROLE_TWITCH_BOT_BYTES: [u8; 2] = [0x01, 0x01];
pub const ROLE_DISCORD_BOT_BYTES: [u8; 2] = [0x01, 0x02];
