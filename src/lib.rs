pub use macros::applescript;

#[cfg(test)]
mod tests {
    use macros::applescript;

    #[test]
    fn test_open_url() {
        applescript!(
            tell application "Safari" to open location "https://www.rust-lang.org"
        );
    }
}
