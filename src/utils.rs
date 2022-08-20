use console;

pub fn strip_colors(input: String) -> String {
    return console::strip_ansi_codes(input.as_str()).to_string();
}

pub fn strip_vec_colors(mut input: Vec<String>) -> Vec<String> {
    // Pretty cool piece of in place filtering
    // for evering mutable reference to the items in input
    // set the value behind the reference (how I think the * works)
    // to equal that value put through the function 'strip_colors'
    for i in &mut input {
        *i = strip_colors((*i).to_string());
    }
    return input;
}
