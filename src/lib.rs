pub enum Message {
    Blank,
    PlayCard,
}

pub fn parse_message(message: String) -> Message {

    if message == "go play-card" {
        Message::PlayCard
    } else {
        Message::Blank
    }
}

#[cfg(test)]
mod test_parsing_messages{
    use super::parse_message;
    use super::Message;

    #[test]
    fn play_card_message() {
        let x = parse_message(String::from("go play-card"));
        match x {
            Message::PlayCard => {
            },
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn unknown_message_is_blank() {
        let x = parse_message(String::from("this isn't a message"));
        match x {
            Message::Blank => {
            },
            _ => panic!("Wrong Card type."),
        }
    }
}
