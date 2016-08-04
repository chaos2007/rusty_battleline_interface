#![feature(advanced_slice_patterns, slice_patterns)]
extern crate regex;
use regex::Regex;

pub struct Response {
    response: Option<String>
}

pub trait ai_handler {
    fn handleMessage(&self, Message) -> Option<String>;
}

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
    ColorNames,//{ nums: [String]},
    FlagClaimStatus,//{ nums: [ClaimStatus]},
    FlagStatus,//{ number, direction, cards},
    OpponentPlay{ number: i32, card: Card },
    PlayCard,
}

pub fn parse_message(message: String) -> Message {
/*    let opMessage = Regex::new(r"opponent play (?P<flag>[1-9]) (?P<color>\S+),(?P<number>\d+)").unwrap();
    let colorsMessage = Regex::new(r"colors( \S+){6}").unwrap();
    let playerHandMessage = Regex::new(r"player (north|south) hand( \S+,\d+)*").unwrap();
    let claimStatusMessage = Regex::new(r"flag claim-status( north| south| unclaimed){9}").unwrap();
    let flagCardsMessage = Regex::new(r"flag ([1-9]) cards (north|south)( \S+,\d+)*").unwrap();
    */
    let split: Vec<&str> = message.split_whitespace().collect();
    match split.as_slice() {
        &["go", "play-card"] => Message::PlayCard,
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
            Message::OpponentPlay{number: number, card: card} => {
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
}

