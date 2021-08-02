use crate::Request;

pub fn preprocess(input: &str) -> &str {
    let input = input.trim_start();
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(&input)
}

pub fn parse(input: &str) -> Option<Request> {
    let (command, message) = match input.split_once(' ') {
        Some(content) => content,
        None => (input, ""),
    };

    match command.to_ascii_lowercase().as_str() {
        "echo" => Some(Request::Echo(message.into())),
        "exit" => Some(Request::Exit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preprocess_regular() {
        assert_eq!("echo Hello!", preprocess("echo Hello!\r\n"));
    }

    #[test]
    fn preprocess_leading_whitespace() {
        assert_eq!("exit ", preprocess("    exit \n"));
    }

    #[test]
    fn preprocess_multi_newline() {
        assert_eq!("echo Hello!\n", preprocess("echo Hello!\n\n"));
    }

    #[test]
    fn parse_echo() {
        assert_eq!(Some(Request::Echo("Hello!".into())), parse("echo Hello!"));
    }

    #[test]
    fn parse_exit() {
        assert_eq!(Some(Request::Exit), parse("exit "));
    }

    #[test]
    fn parse_invalid() {
        assert_eq!(None, parse("something 123 abc"));
    }
}
