use clap::{App, AppSettings};

pub fn parse_arguments() {
    app_from_crate!()
        .usage(format!("{} [FLAGS/OPTIONS]", env!("CARGO_PKG_NAME")).as_str())
        .setting(AppSettings::ColorAlways)
        .get_matches();
}
