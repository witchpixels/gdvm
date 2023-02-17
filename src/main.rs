use crate::version_sourcing::tux_family::tux_family_version_source::download_godot_versions_from_tux_family;

mod version_sourcing;

fn main() {
    let versions = download_godot_versions_from_tux_family();
    for version in versions {
        println!(
            "{} {} {}",
            version.version_tag, version.mono, version.download_url
        );
    }
}
