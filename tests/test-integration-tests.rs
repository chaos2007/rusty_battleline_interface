extern crate rusty_battleline_interface as rbi;

#[test]
fn confirm_parsing_message() {
    let x = rbi::message_parsing::parse_message(String::from("go play-card"));
    match x {
        rbi::message_parsing::Message::PlayCard => {
        },
        _ => panic!("Wrong Card type."),
    }
}


