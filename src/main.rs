use regex::Regex;

pub fn extract_godot_directory_versions(html_response: &String) -> Vec<String> {

    let mut vec: Vec<String> = Vec::new();

    let regex = Regex::new(r#"="([0-9]\.[0-9]\.?[0-9]?\.?[0-9]?)/""#).unwrap();
    for capture in regex.captures_iter(&html_response) {
        vec.push(capture[1].to_string());
    }

    return vec;
}

fn main() {

    let resp = reqwest::blocking::get("https://downloads.tuxfamily.org/godotengine/")
        .unwrap()
        .text()
        .unwrap();

    let versions = extract_godot_directory_versions(&resp);

    for version in versions{
        println!("found version {}", version);
    }

}
