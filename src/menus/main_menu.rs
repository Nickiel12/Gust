use colored::Colorize;
use console::Term;

pub fn main_menu() -> Result<usize, std::io::Error> {
    let term = Term::stdout();
    term.write_line("Hello World!")?;

    term.clear_line()?;

    Ok(1)
}
