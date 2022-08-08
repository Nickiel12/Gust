use regex;

pub fn strip_colors(input: String) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*[mGKHF]").unwrap();
    return re.replace_all(input.as_str(), "").to_string();
}
