use std::io::Write;

pub fn prompt(message: &str) {
    let mut stdout = std::io::stdout();
    if let Err(e) = stdout.write_all(message.as_bytes()) {
        panic!("{e}");
    };
    stdout.flush().expect("stdout flush cannot be performed");
}

pub fn format_name(name: &str) -> String {
    name.replace('_', " ")
}