pub fn reply(message: &str) -> &str {
    match (
        message.trim().chars().last().unwrap_or('.') == '?',
        message.matches(char::is_alphabetic).count() == message.matches(char::is_uppercase).count()
            && message.matches(char::is_alphabetic).count() > 0,
        message.trim().is_empty(),
    ) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
