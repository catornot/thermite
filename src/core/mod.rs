#[cfg(feature = "manage")]
pub mod manage;
#[cfg(feature = "utils")]
#[allow(dead_code)]
pub mod utils;

#[cfg(all(target_os = "linux", feature = "proton", feature = "utils"))]
pub use utils::proton::{download_ns_proton, install_ns_proton, latest_release};
#[cfg(all(feature = "steam", feature = "utils"))]
pub use utils::steam::{steam_dir, steam_libraries, titanfall};
#[cfg(feature = "utils")]
pub use utils::{find_mods, get_enabled_mods, resolve_deps};
