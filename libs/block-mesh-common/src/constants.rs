use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

pub const DEV_ENV: [&str; 3] = ["dev", "development", "local"];
pub const BLOCKMESH_TWITTER_USER_ID: u64 = 1766124448778784768;
pub const BLOCKMESH_FOUNDER_TWITTER_USER_ID: u64 = 1778711300127821824;
pub const UFBOTS_TWITTER_ID: u64 = 1902284045574402049;

pub const PERCEPTRON_NTWK_TWITTER_ID: u64 = 1880853313488609280;

pub const MRRYDON_TWITTER_ID: u64 = 814974533769662464;

pub const PETER_THOC_TWITTER_ID: u64 = 1903384295001010176;
pub const FRODOBOTS_TWITTER_ID: u64 = 1493135152024678401;
pub const SAM_IS_MOVING_TWITTER_ID: u64 = 1853818882332434432;
pub const BIT_ROBOT_TWITTER_ID: u64 = 1861258248483151874;
pub const ROBOTS_DOT_FUN_ID: u64 = 1861269701537734658;
pub const XENO_TWITTER_USER_ID: u64 = 1851306491732709376;
pub const WOOTZ_APP_USER_ID: u64 = 1434571586829357057;
pub const BLOCKMESH_PG_NOTIFY_WORKER: &str = "pgchannel";
pub const BLOCKMESH_PG_NOTIFY_API: &str = "pgchannel_api";
pub const BLOCKMESH_PG_NOTIFY_EMAIL: &str = "pgchannel_email";
pub const BLOCKMESH_VPS: &str = "https://vps.blockmesh.xyz";

pub const BLOCKMESH_WS_REDIS_COUNT_KEY: &str = "BLOCKMESH_WS_REDIS_COUNT_KEY";
pub const CF_TURNSTILE: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
pub const RECAPTCHA: &str = "https://www.google.com/recaptcha/api/siteverify";
pub const HCAPTCHA: &str = "https://api.hcaptcha.com/siteverify";

pub fn env_url() -> String {
    static APP_ENVIRONMENT: OnceLock<String> = OnceLock::new();
    let app_environment =
        APP_ENVIRONMENT.get_or_init(|| env::var("APP_ENVIRONMENT").unwrap_or_default());
    if DEV_ENV.contains(&app_environment.as_str()) {
        "http://localhost:8000".to_string()
    } else {
        "https://app.blockmesh.xyz".to_string()
    }
}
pub static BLOCK_MESH_SUPPORT_CHAT: &str = "https://blockmesh.xyz/support-chat";
pub static BLOCK_MESH_EMAILS: &str = "https://emails.blockmesh.xyz";

pub static BLOCK_MESH_FEATURE_FLAGS: &str = "https://feature-flags.blockmesh.xyz";
pub static BLOCK_MESH_SUPPORT_EMAIL: &str = "support@blockmesh.xyz";
pub static PCN_LOGO: &str = "https://perceptron-network.perceptrons.xyz/Logo_only_white.png";

pub static BLOCK_MESH_LANDING_PAGE_IMAGE: &str =
    "https://r2-images.blockmesh.xyz/dfda0dd2-a321-4c75-cbbe-5521b2355f00.png";
pub static BLOCK_MESH_IP_WORKER: &str = "https://cloudflare-worker-ip-data.blockmesh.xyz/";
pub static BLOCK_MESH_PROGRAM_ID: &str = "FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ";
// pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logs-queue.blockmesh.xyz";
pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logger-proxy.blockmesh.xyz";

pub static BLOCK_MESH_CHROME_EXTENSION_LINK: &str =
    "https://chromewebstore.google.com/detail/perceptron-network/dflhdcckcmcajgofmipokpgknmfikhej";

pub static BLOCK_MESH_APP_SERVER: &str = "https://app.blockmesh.xyz";
pub static BLOCK_MESH_API_SERVER: &str = "https://api.blockmesh.xyz";

pub static BLOCK_MESH_GITHUB: &str = "https://github.com/block-mesh/block-mesh-monorepo";

pub static BLOCK_MESH_TWITTER: &str = "https://twitter.com/blockmesh_xyz";

pub static BLOCK_MESH_GITBOOK: &str = "https://gitbook.blockmesh.xyz/";

pub static BLOCKMESH_LOG_ENV: &str = "BLOCKMESH_LOG_ENV";

pub static BLOCKMESH_HOME_DIR_ENVAR: &str = "BLOCKMESH_HOME_DIR";
pub static BLOCKMESH_DISABLE_GUI_ENVAR: &str = "BLOCKMESH_DISABLE_GUI";

pub static CONFIG_FILENAME: &str = "blockmesh.json";

pub static BLOCKMESH_SERVER_UUID_ENVAR: &str = "BLOCKMESH_SERVER_UUID";

pub static BLOCKMESH_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Extension,
    Desktop,
    Mobile,
    Tablet,
    Unknown,
    TestProxyMaster,
    TestProxyEndpoint,
    AppServer,
    Cli,
    Worker,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Extension => write!(f, "extension"),
            DeviceType::Desktop => write!(f, "desktop"),
            DeviceType::Mobile => write!(f, "mobile"),
            DeviceType::Tablet => write!(f, "tablet"),
            DeviceType::Unknown => write!(f, "unknown"),
            DeviceType::TestProxyMaster => write!(f, "test-proxy-master"),
            DeviceType::TestProxyEndpoint => write!(f, "test-proxy-endpoint"),
            DeviceType::AppServer => write!(f, "app-server"),
            DeviceType::Cli => write!(f, "cli"),
            DeviceType::Worker => write!(f, "worker"),
        }
    }
}

impl From<&str> for DeviceType {
    fn from(value: &str) -> Self {
        match value {
            "extension" => DeviceType::Extension,
            "desktop" => DeviceType::Desktop,
            "mobile" => DeviceType::Mobile,
            "tablet" => DeviceType::Tablet,
            "unknown" => DeviceType::Unknown,
            "test-proxy-master" => DeviceType::TestProxyMaster,
            "test-proxy-endpoint" => DeviceType::TestProxyEndpoint,
            "app-server" => DeviceType::AppServer,
            "cli" => DeviceType::Cli,
            "worker" => DeviceType::Worker,
            _ => DeviceType::Unknown,
        }
    }
}

impl From<String> for DeviceType {
    fn from(value: String) -> Self {
        DeviceType::from(value.as_str())
    }
}

pub enum RankBonus {
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
    Grandmaster,
    Legend,
}

impl From<RankBonus> for f64 {
    fn from(val: RankBonus) -> f64 {
        match val {
            RankBonus::Novice => 25_000.0,
            RankBonus::Apprentice => 50_000.0,
            RankBonus::Journeyman => 100_000.0,
            RankBonus::Expert => 200_000.0,
            RankBonus::Master => 500_000.0,
            RankBonus::Grandmaster => 800_000.0,
            RankBonus::Legend => 1_000_000.0,
        }
    }
}

pub const BUTTON_CLASS: &str ="text-magenta-2 -my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default";

pub const INTRACT_USER_INFO_API_URL: &str =
    "https://publicapi.intract.io/api/pv1/proof-of-humanity/user-info";
