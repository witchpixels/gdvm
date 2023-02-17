pub(crate) mod tux_family;

#[derive(Debug)]
pub enum Platform {
    Linux,
    //Mac,
    //Windows,
}

#[derive(Debug)]
pub struct AvailableVersion {
    pub version_tag: semver::Version,
    pub download_url: String,
    pub platform: Platform,
    pub mono: bool,
}
