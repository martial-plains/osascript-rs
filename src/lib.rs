pub use macros::__applescript;
use std::process::Command;

pub fn run_applescript(script: &str) {
    let process = Command::new("osascript")
        .arg("-l")
        .arg("AppleScript")
        .arg("-e")
        .arg(script)
        .spawn()
        .expect("Failed to run AppleScript");

    let _ = process.wait_with_output().unwrap();
}

#[macro_export]
macro_rules! applescript {
    ($($script: tt)*) => {{
        use $crate::{run_applescript, __applescript};

        __applescript!($($script)*)
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_open_url() {
        applescript!(
            tell application "Safari" to open location "https://www.rust-lang.org"
        );
    }

    #[test]
    fn test_multiline_works() {
        applescript! {
            tell application "Safari"
                activate
                try
                    tell window 1 to set current tab to make new tab with properties {URL:"https://www.rust-lang.org"}
                on error
                    open location theURL
                end try
            end tell
        }
    }
}
