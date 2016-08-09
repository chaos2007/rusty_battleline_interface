pub enum Direction {
    North,
    South,
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
    PlayerDirection {
        direction: Direction,
    },
    ColorNames {
        c1: String,
        c2: String,
        c3: String,
        c4: String,
        c5: String,
        c6: String,
    },
    FlagClaimStatus {
        flagsClaimed: Vec<ClaimStatus>,
    },
    FlagStatus {
        flagNum: u32,
        direction: Direction,
        cards: Vec<Card>,
    },
    OpponentPlay {
        number: i32,
        card: Card,
    },
    PlayerHand {
        direction: Direction,
        cards: Vec<Card>,
    },
    PlayCard,
}

fn convert_direction(message: &str) -> Direction {
    match message {
        "north" => Direction::North,
        "south" => Direction::South,
        e => panic!("{} is not a direction flag.", e),
    }
}

pub fn get_direction_string(direction: Direction) -> String {
    match direction {
        Direction::North => String::from("north"),
        Direction::South => String::from("south"),
    }
}


fn convert_claim_status(message: &str) -> ClaimStatus {
    match message {
        "north" => ClaimStatus::North,
        "south" => ClaimStatus::South,
        "unclaimed" => ClaimStatus::Unclaimed,
        e => panic!("{} is not a Claim Status flag.", e),
    }
}

fn convert_string_to_card(message: &str) -> Card {
    let split: Vec<&str> = message.split(',').collect();
    let b = String::from(split[0]);
    let c = split[1].parse::<i32>().unwrap();
    Card {
        color: b,
        number: c,
    }
}

pub fn parse_message(message: String) -> Message {
    let split: Vec<&str> = message.split_whitespace().collect();
    match &split[..] {
        &["go", "play-card"] => Message::PlayCard,
        &["player", direction, "hand", ref cards..] if cards.len() <= 7 &&
                                                       (direction == "north" ||
                                                        direction == "south") => {
            let mut playerCards = Vec::new();
            for i in cards {
                playerCards.push(convert_string_to_card(i));
            }
            Message::PlayerHand {
                direction: convert_direction(direction),
                cards: playerCards,
            }
        }
        &["flag", flag, "cards", direction, ref cards..] if flag >= "1" && flag <= "9" &&
                                                            (direction == "north" ||
                                                             direction == "south") => {
            let mut flagsCards = Vec::new();
            let a = flag.parse::<u32>().unwrap();
            for i in cards {
                flagsCards.push(convert_string_to_card(i));
            }
            Message::FlagStatus {
                flagNum: a,
                direction: convert_direction(direction),
                cards: flagsCards,
            }
        }
        &["player", direction, "name"] if direction == "north" || direction == "south" => {
            match direction {
                "north" => Message::PlayerDirection { direction: Direction::North },
                _ => Message::PlayerDirection { direction: Direction::South },
            }
        }
        &["opponent", "play", flag, card] if flag >= "1" && flag <= "9" => {
            let a = flag.parse::<i32>().unwrap();
            Message::OpponentPlay {
                number: a,
                card: convert_string_to_card(card),
            }
        }
        &["colors", c1, c2, c3, c4, c5, c6] => {
            Message::ColorNames {
                c1: String::from(c1),
                c2: String::from(c2),
                c3: String::from(c3),
                c4: String::from(c4),
                c5: String::from(c5),
                c6: String::from(c6),
            }
        }
        &["flag", "claim-status", ref flagClaims..] if flagClaims.len() == 9 => {
            let mut claims = Vec::new();
            for i in flagClaims {
                claims.push(convert_claim_status(i));
            }
            Message::FlagClaimStatus { flagsClaimed: claims }
        }
        &["player", direction, hand, ref cards..] => Message::Blank,
        &["flag", number, "cards", direction, ref cards..] => Message::Blank,
        _ => Message::Blank,
    }
}

#[cfg(test)]
mod test_parsing_messages {
    use super::*;

    #[test]
    fn play_card_message() {
        let x = parse_message(String::from("go play-card"));
        match x {
            Message::PlayCard => {}
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn unknown_message_is_blank() {
        let x = parse_message(String::from("this isn't a message"));
        match x {
            Message::Blank => {}
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn opponent_play_message() {
        let x = parse_message(String::from("opponent play 3 red,5"));
        match x {
            Message::OpponentPlay { number, card } => {
                assert_eq!(String::from("red"), card.color);
                assert_eq!(5, card.number);
                assert_eq!(3, number);
            }
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn player_direction_message() {
        let x = parse_message(String::from("player south name"));
        match x {
            Message::PlayerDirection { direction: Direction::South } => {}
            _ => panic!("Wrong Card type."),
        }
        let x = parse_message(String::from("player north name"));
        match x {
            Message::PlayerDirection { direction: Direction::North } => {}
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn color_names_message() {
        let x = parse_message(String::from("colors a b c d e f"));
        match x {
            Message::ColorNames { ref c1, ref c2, ref c3, ref c4, ref c5, ref c6 } if c1 == "a" &&
                                                                                      c2 == "b" &&
                                                                                      c3 == "c" &&
                                                                                      c4 == "d" &&
                                                                                      c5 == "e" &&
                                                                                      c6 == "f" => {
            }
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn flag_claim_message() {
        let claimVec = vec![ClaimStatus::North,
                            ClaimStatus::South,
                            ClaimStatus::Unclaimed,
                            ClaimStatus::Unclaimed,
                            ClaimStatus::South,
                            ClaimStatus::North,
                            ClaimStatus::South,
                            ClaimStatus::North,
                            ClaimStatus::Unclaimed];

        let x = parse_message(String::from("flag claim-status north south unclaimed unclaimed \
                                            south north south north unclaimed"));
        match x {
            Message::FlagClaimStatus { flagsClaimed: claimVec } => {}
            e => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn flag_card_message() {
        let cards = vec![Card {
                             color: String::from("red"),
                             number: 3,
                         },
                         Card {
                             color: String::from("blue"),
                             number: 9,
                         }];
        let x = parse_message(String::from("flag 3 cards north red,3 blue,9"));
        match x {
            Message::FlagStatus { flagNum: 3, direction: Direction::North, cards: cards } => {}
            _ => panic!("Wrong Card type."),
        }
    }

    #[test]
    fn player_hand_message() {
        let cards = vec![Card {
                             color: String::from("red"),
                             number: 3,
                         },
                         Card {
                             color: String::from("blue"),
                             number: 6,
                         },
                         Card {
                             color: String::from("green"),
                             number: 4,
                         }];
        let x = parse_message(String::from("player north hand red,3 blue,6 green,4"));
        match x {
            Message::PlayerHand { direction: Direction::North, cards: cards } => {}
            _ => panic!("Wrong Card type."),
        }
    }
}
