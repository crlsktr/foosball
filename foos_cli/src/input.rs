    use std::io;
    use std::io::Write;

    pub fn get_input(prompt: &str) -> String{
        print!("{}",prompt);
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {},
        }
        input.trim().to_string()
    }