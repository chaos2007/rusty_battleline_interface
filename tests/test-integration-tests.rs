extern crate rusty_battleline_interface as rbi;

#[test]
fn confirmParsingMessage() {
    let x = rbi::parse_message(String::from("go play-card"));
    match x {
        rbi::Message::PlayCard => {
        },
        _ => panic!("Wrong Card type."),
    }
}


