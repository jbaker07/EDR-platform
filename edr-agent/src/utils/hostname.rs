use std::ffi::OsString;

/// Returns the current system hostname as a String
pub fn get_hostname() -> String {
    hostname::get()
        .unwrap_or_else(|_| OsString::from("unknown-host"))
        .to_string_lossy()
        .into_owned()
}
