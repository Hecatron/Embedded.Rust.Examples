// See - https://github.com/rp-rs/rp-hal/issues/910

/// Generate a static item containing the `CARGO_BIN_NAME` as the program name,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_cargo_bin_name {
    () => {
        hal::binary_info::env!(
            rp_binary_info::consts::TAG_RASPBERRY_PI,
            rp_binary_info::consts::ID_RP_PROGRAM_NAME,
            "CARGO_BIN_NAME"
        )
    };
}

/// Generate a static item containing the `CARGO_PKG_HOMEPAGE` as the program URL,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_cargo_homepage_url {
    () => {
        rp_binary_info::env!(
            rp_binary_info::consts::TAG_RASPBERRY_PI,
            rp_binary_info::consts::ID_RP_PROGRAM_URL,
            "CARGO_PKG_HOMEPAGE"
        )
    };
}
