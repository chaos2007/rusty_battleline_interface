#![feature(advanced_slice_patterns, slice_patterns)]

pub enum Direction {
    North,
    South
}

pub struct Card {
    color: String,
    number: i32,
}

pub enum ClaimStatus {
    Unclaimed,
    North,
    South,
}


pub enum Message {
    Blank,
    PlayerDirection{ direction: Direction},
    ColorNames{ c1:String, c2:String, c3:String, c4:String, c5:String, c6:String },
    FlagClaimStatus{ c1:ClaimStatus, c2:ClaimStatus, c3:ClaimStatus, 
        c4:ClaimStatus, c5:ClaimStatus, c6:ClaimStatus, 
        c7:ClaimStatus, c8:ClaimStatus, c9:ClaimStatus, },
    FlagStatus,//{ number, direction, cards},
    OpponentPlay{ number: i32, card: Card },
    PlayerHandMessage{ direction: Direction, cards: Vec<Card> },
    PlayCard,
}

fn convert_claim_status(message: &str) -> ClaimStatus {
    match message {
        "north" => ClaimStatus::North,
        "south" => ClaimStatus::South,
        "unclaimed" => ClaimStatus::Unclaimed,
        e => panic!("{} is not a Claim Status flag.", e),
    }
}

pub fn parse_message(message: String) -> Message {
/*    
    let playerHandMessage = Regex::new(r"player (north|south) hand( \S+,\d+)*").unwrap();
    let flagCardsMessage = Regex::new(r"flag ([1-9]) cards (north|south)( \S+,\d+)*").unwrap();
    */
    let split: Vec<&str> = message.split_whitespace().collect();
    match split.as_slice() {
        &["go", "play-card"] => Message::PlayCard,
        &["player", direction, "hand", ref hand..] => {
            for x in hand {
                println!("{}",x);
            }

            Message::Blank
        },
        &["player", direction, "name"] if direction == "north" || direction == "south" => {
            match direction {
                "north" => Message::PlayerDirection{ direction: Direction::North },
                _ => Message::PlayerDirection{ direction: Direction::South}
            }
        },
        &["opponent", "play", flag, card] if flag >= "1" && flag <= "9" => {
            let a = flag.parse::<i32>().unwrap();
            let split: Vec<&str>  = card.split(',').collect();
            let b = String::from(split[0]);
            let c = split[1].parse::<i32>().unwrap();
            Message::OpponentPlay{ number: a, card: Card{ color: b, number:c}}
        },
        &["colors", c1,c2,c3,c4,c5,c6] => {
            Message::ColorNames{ c1:String::from(c1), c2:String::from(c2), 
                c3:String::from(c3), c4:String::from(c4), c5:String::from(c5), 
                c6:String::from(c6)}
        },
        &["flag", "claim-status", c1,c2,c3,c4,c5,c6,c7,c8,c9] => {
            let c1 = convert_claim_status(c1);
            let c2 = convert_claim_status(c2);
            let c3 = convert_claim_status(c3);
            let c4 = convert_claim_status(c4);
            let c5 = convert_claim_status(c5);
            let c6 = convert_claim_status(c6);
            let c7 = convert_claim_status(c7);
            let c8 = convert_claim_status(c8);
            let c9 = convert_claim_status(c9);
            Message::FlagClaimStatus{c1:c1, c2:c2, c3:c3, c4:c4, c5:c5, c6:c6, c7:c7, c8:c8, c9:c9}
        },
        _ => Message::Blank,
    }
}

#[cfg(test)]
mod test_parsing_messages{
    use super::*;

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
    
    #[test]
    fn opponent_play_message() {
        let x = parse_message(String::from("opponent play 3 red,5"));
        match x {
            Message::OpponentPlay{number, card} => {
                assert_eq!(String::from("red"), card.color);
                assert_eq!(5, card.number);
                assert_eq!(3, number);
            },
            _ => panic!("Wrong Card type."),
        }
    }
    
    #[test]
    fn player_direction_message() {
        let x = parse_message(String::from("player south name"));
        match x {
            Message::PlayerDirection{direction: Direction::South} => {
            },
            _ => panic!("Wrong Card type."),
        }
        let x = parse_message(String::from("player north name"));
        match x {
            Message::PlayerDirection{direction: Direction::North} => {
            },
            _ => panic!("Wrong Card type."),
        }
    }
    
    #[test]
    fn color_names_message() {
        let x = parse_message(String::from("colors a b c d e f"));
        match x {
            Message::ColorNames{ref c1,ref c2, ref c3, ref c4,ref c5,ref c6} if 
                c1 == "a" && c2 == "b" && c3 == "c" && 
                c4 == "d" && c5 == "e" && c6 == "f" => {
            },
            _ => panic!("Wrong Card type."),
        }
    }
    
    #[test]
    fn flag_claim_message() {
        let x = parse_message(String::from("flag claim-status north south unclaimed unclaimed south north south north unclaimed"));
        match x {
            Message::FlagClaimStatus{ c1:ClaimStatus::North, c2:ClaimStatus::South,
                c3:ClaimStatus::Unclaimed, c4:ClaimStatus::Unclaimed, c5:ClaimStatus::South,
                c6:ClaimStatus::North, c7:ClaimStatus::South,
                c8:ClaimStatus::North, c9:ClaimStatus::Unclaimed } => {
                },
            e => panic!("Wrong Card type."),
        }
    }
    
    #[test]
    fn player_hand_message() {
        let x = parse_message(String::from("player north hand red,3 blue,4"));
        let mut vec = Vec::new(); vec.push(Card{color:String::from("red"),number:3});
        vec.push(Card{color:String::from("blue"),number:4});
        match x {
            Message::PlayerHandMessage{direction:Direction::North, cards: vec } => {
            }, 
            e => panic!("Wrong Card type."),
        }
    }
}

