//! Provides the `ConnectInstruction` structure, which allows for a `ParsedDisplay`
//! to be transformed into a server connection.

use super::ParsedDisplay;
use std::path::PathBuf;

/// Get an iterator over all of the addresses we should target with a
/// `ParsedDisplay`.
pub(super) fn connect_addresses(p: &ParsedDisplay) -> Option<PathBuf> {
    let ParsedDisplay {
        protocol, display, ..
    } = p;

    if protocol.is_none() || protocol.as_deref() == Some("unix") {
        let file_name = format!("/tmp/.X11-unix/X{}", display);

        // TODO: Try abstract socket (file name with prepended '\0')
        // Not supported on Rust right now: https://github.com/rust-lang/rust/issues/42048
        Some(file_name.into())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // make sure iterator properties are clean
    use super::super::parse_display;
    use std::path::PathBuf;

    #[test]
    fn basic_test() {
        let pd = parse_display(Some(":0")).unwrap();
        let ci = pd.connect_instruction();
        let ci = ci.unwrap();

        assert_eq!(ci, PathBuf::from("/tmp/.X11-unix/X0"),);
    }

    #[test]
    fn try_over_unix_hostname() {
        let pd = parse_display(Some("unix/host:0")).unwrap();
        let ci = pd.connect_instruction();

        let ci = ci.unwrap();

        assert_eq!(ci, PathBuf::from("/tmp/.X11-unix/X0"),);
    }
}
