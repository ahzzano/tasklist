use std::io::Write;

pub fn read_console_line(prompt: String) -> String {
    let mut line = String::new();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    line.trim_end().to_string()
}
