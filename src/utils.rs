use console;

pub fn strip_colors(input: String) -> String {
    return console::strip_ansi_codes(input.as_str()).to_string();
}
