use regex::Regex;

pub fn extract_godot_root_directory_versions(html_response: &String) -> Vec<String> {
    let regex = Regex::new(r#"="([0-9]\.[0-9]\.?[0-9]?\.?[0-9]?)/""#).unwrap();
    return extract_capture(regex, html_response);
}

pub fn extract_godot_directory_versions(html_response: &String) -> Vec<String> {
    let regex = Regex::new(r#"href="([A-Za-z.0-9]+\w)/""#).unwrap();
    return extract_capture(regex, html_response);
}

#[derive(Debug)]
pub struct GodotDownload {
    pub version_number: String,
    pub version_tag: String,
    pub platform: String,
    pub file_path: String,
}

pub fn extract_godot_downloadable_files(html_response: &String) -> Vec<GodotDownload> {
    let regex = Regex::new(r#""[G|g]odot_v([0-9.]+)-([A-z0-9]+)_([A-z_0-9.]+).zip">([A-z0-9.-]+)"#)
        .unwrap();

    let mut vec: Vec<GodotDownload> = Vec::new();

    for capture in regex.captures_iter(&html_response) {
        vec.push(GodotDownload {
            version_number: capture[1].to_string(),
            version_tag: capture[2].to_string(),
            platform: capture[3].to_string(),
            file_path: capture[4].to_string(),
        })
    }

    return vec;
}

fn extract_capture(regex: Regex, body: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for capture in regex.captures_iter(&body) {
        vec.push(capture[1].to_string());
    }

    return vec;
}
