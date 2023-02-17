use crate::version_sourcing::tux_family::regex_helpers;
use crate::version_sourcing::tux_family::regex_helpers::{
    extract_godot_directory_versions, extract_godot_downloadable_files, GodotDownload,
};
use crate::version_sourcing::{AvailableVersion, Platform};

pub fn download_godot_versions_from_tux_family() -> Vec<AvailableVersion> {
    let mut vec: Vec<AvailableVersion> = Vec::new();

    let tux_family_url = "https://downloads.tuxfamily.org/godotengine";
    let resp = reqwest::blocking::get(tux_family_url)
        .unwrap()
        .text()
        .unwrap();

    let version_roots = regex_helpers::extract_godot_root_directory_versions(&resp);

    for version_root in version_roots {
        let sub_vec = crawl_directory(&format!("{tux_family_url}/{version_root}"), false);

        for av in sub_vec {
            vec.push(av);
        }
    }

    return vec;
}

fn crawl_directory(url: &String, mono: bool) -> Vec<AvailableVersion> {
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    let mut vec: Vec<AvailableVersion> = Vec::new();

    let directories = extract_godot_directory_versions(&resp);

    for sub_dir in directories {
        let sub_is_mono = sub_dir.eq_ignore_ascii_case("mono");

        let sub_vec = crawl_directory(&format!("{url}/{sub_dir}"), sub_is_mono);

        for av in sub_vec {
            vec.push(av);
        }
    }

    let files = extract_godot_downloadable_files(&resp);

    for file in files {
        vec.push(crack_file_capture(url, mono, file));
    }

    return vec;
}

fn crack_file_capture(url: &String, is_mono: bool, capture: GodotDownload) -> AvailableVersion {
    println!("Processing {:?}", capture);

    let mut version = match semver::Version::parse(&capture.version_number) {
        Ok(v) => v,
        Err(_) => semver::Version::parse(&(capture.version_number + ".0")).unwrap(),
    };

    if !capture.version_tag.contains("[S|s]table") {
        version.pre =
            semver::Prerelease::new(&capture.version_tag.split("_").next().unwrap()).unwrap();
    }

    return AvailableVersion {
        version_tag: version,
        download_url: format!("{url}/{}", capture.file_path),
        platform: Platform::Linux,
        mono: is_mono,
    };
}
