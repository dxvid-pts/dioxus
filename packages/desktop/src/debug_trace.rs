//! Debug-only file logging for Windows white-screen investigations.

#[cfg(all(debug_assertions, windows))]
use std::{
    io::Write,
    path::{Path, PathBuf},
};

#[cfg_attr(not(all(debug_assertions, windows)), allow(dead_code))]
pub(crate) const LOG_PREFIX: &str = "[SLDX_DIOXDBG]";

macro_rules! dioxdbg {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(windows)]
            {
                let message = $crate::debug_trace::prefixed_message(&format!($($arg)*));
                $crate::debug_trace::try_log(&message);
            }

            #[cfg(not(windows))]
            {
                let _ = format!($($arg)*);
            }
        }
    };
}

pub(crate) use dioxdbg;

#[cfg_attr(not(all(debug_assertions, windows)), allow(dead_code))]
pub(crate) fn prefixed_message(message: &str) -> String {
    format!("{LOG_PREFIX} {message}")
}

#[cfg(all(debug_assertions, windows))]
pub(crate) fn try_log(message: &str) {
    let timestamp = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };

    let line = format!("[{timestamp}] {message}\n");
    let log_path = try_log_path();
    let open = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path);

    if let Ok(mut file) = open {
        let _ = file.write_all(line.as_bytes());
    }
}

#[cfg(all(debug_assertions, windows))]
fn try_log_path() -> PathBuf {
    match std::env::var("USERPROFILE") {
        Ok(profile_dir) => try_downloads_log_path(Path::new(&profile_dir)),
        Err(_) => std::env::temp_dir().join("slidex_debug.log"),
    }
}

#[cfg(all(debug_assertions, windows))]
fn try_downloads_log_path(base_dir: &Path) -> PathBuf {
    base_dir.join("Downloads").join("slidex_debug.log")
}

#[cfg(test)]
mod tests {
    use super::{LOG_PREFIX, prefixed_message};

    #[test]
    fn prefixed_message_uses_stable_prefix() {
        assert_eq!(
            prefixed_message("window_created"),
            format!("{LOG_PREFIX} window_created")
        );
    }
}
