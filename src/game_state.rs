use message_parsing;

pub trait AiInterface {
    fn update_game_state(&self, state: &GameState) -> String;
    fn get_bot_name(&self) -> String;
}

#[derive(Default, Debug)]
pub struct GameState {
    pub player_direction: message_parsing::Direction,
    pub opponent_direction: message_parsing::Direction,
    pub deck: Vec<message_parsing::Card>,
    pub colors: Vec<String>,
    pub colors_vec: Vec<message_parsing::Color>,
    pub claim_status: Vec<message_parsing::ClaimStatus>,
    pub opponent_side: Vec<Vec<message_parsing::Card>>,
    pub player_side: Vec<Vec<message_parsing::Card>>,
    pub player_hand: Vec<message_parsing::Card>,
}

impl GameState {
    pub fn color_from_string(&self, name: &String) -> message_parsing::Color {
        if let Some(index) = self.colors.iter().position(|i| *i == *name) {
            match index {
                0 => message_parsing::Color::Color1,
                1 => message_parsing::Color::Color2,
                2 => message_parsing::Color::Color3,
                3 => message_parsing::Color::Color4,
                4 => message_parsing::Color::Color5,
                _ => message_parsing::Color::Color6,
            }
        } else {
            message_parsing::Color::Color1
        }
    }
    pub fn string_from_color(&self, color: message_parsing::Color) -> String {
        match color {
            message_parsing::Color::Color1 => self.colors[0].clone(),
            message_parsing::Color::Color2 => self.colors[1].clone(),
            message_parsing::Color::Color3 => self.colors[2].clone(),
            message_parsing::Color::Color4 => self.colors[3].clone(),
            message_parsing::Color::Color5 => self.colors[4].clone(),
            message_parsing::Color::Color6 => self.colors[5].clone(),
        }
    }
    pub fn convert_card_string_to_card(&self,
                                       cs: &message_parsing::CardString)
                                       -> message_parsing::Card {
        message_parsing::Card {
            number: cs.number,
            color: self.color_from_string(&cs.color),
        }
    }
    pub fn convert_vector_card_string_to_cards(&self,
                                               cs: &Vec<message_parsing::CardString>)
                                               -> Vec<message_parsing::Card> {
        let mut card_vec: Vec<message_parsing::Card> = vec![];
        for card in cs {
            card_vec.push(self.convert_card_string_to_card(card));
        }
        card_vec
    }
}

#[derive(Default)]
pub struct GameHandler {
    pub state: GameState,
}


impl GameHandler {
    pub fn run_one_round(&mut self, ai: &AiInterface, message: String) {
        let x = message_parsing::parse_message(message);
        match x {
            message_parsing::Message::OpponentPlay { number: _, card } => {
                let card = self.state.convert_card_string_to_card(&card);
                if let Some(index) = self.state.deck.iter().position(|i| *i == card) {
                    self.state.deck.remove(index);
                }
            }
            message_parsing::Message::PlayerHand { direction: _, cards } => {
                let cards = self.state.convert_vector_card_string_to_cards(&cards);
                for card in &cards {
                    if let Some(index) = self.state.deck.iter().position(|i| *i == *card) {
                        self.state.deck.remove(index);
                    }
                }
                self.state.player_hand = cards;
            }
            message_parsing::Message::FlagStatus { flag_num: num, direction, cards } => {
                let cards = self.state.convert_vector_card_string_to_cards(&cards);
                for card in &cards {
                    if let Some(index) = self.state.deck.iter().position(|i| *i == *card) {
                        self.state.deck.remove(index);
                    }
                }
                if self.state.player_side.len() == 0 || self.state.opponent_side.len() == 0 {
                    for _ in 1..10 {
                        self.state.player_side.push(vec![]);
                        self.state.opponent_side.push(vec![]);
                    }
                }
                if direction == self.state.player_direction {
                    self.state.player_side[(num - 1) as usize] = cards;
                } else {
                    self.state.opponent_side[(num - 1) as usize] = cards;
                }
            }
            message_parsing::Message::FlagClaimStatus { flags_claimed: claims } => {
                self.state.claim_status = claims;
            }
            message_parsing::Message::PlayerDirection { direction, .. } => {
                self.state.player_direction = if direction == message_parsing::Direction::North {
                    message_parsing::Direction::North
                } else {
                    message_parsing::Direction::South
                };
                self.state.opponent_direction = if direction == message_parsing::Direction::South {
                    message_parsing::Direction::North
                } else {
                    message_parsing::Direction::South
                };
                println!("player {} {}",
                         message_parsing::get_direction_string(direction),
                         ai.get_bot_name());
            }
            message_parsing::Message::ColorNames { colors } => {
                for _ in 1..10 {
                    self.state.player_side.push(vec![]);
                    self.state.opponent_side.push(vec![]);
                    self.state.claim_status.push(message_parsing::ClaimStatus::Unclaimed);
                }

                self.state.colors = colors.clone();
                self.state.colors_vec = vec![];
                for color in &colors {
                    let temp = self.state.color_from_string(color);
                    {
                        self.state.colors_vec.push(temp);
                    }
                }
                for i in 1..10 {
                    for x in colors.to_vec() {
                        let temp = self.state.color_from_string(&x);
                        {
                            self.state.deck.push(message_parsing::Card {
                                color: temp,
                                number: i,
                            })
                        };
                    }
                }
            }
            message_parsing::Message::PlayCard => {
                println!("{}", ai.update_game_state(&self.state));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test_game_state {
    use super::*;
    use message_parsing;
    use message_parsing as mp;

    struct TestAi {
    }

    impl AiInterface for TestAi {
        fn update_game_state(&self, _: &GameState) -> String {
            return String::from("play 1 red,1");
        }

        fn get_bot_name(&self) -> String {
            return String::from("rusty_battleline_bot");
        }
    }

    #[test]
    fn starter_player_check() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        handler.run_one_round(&ai, String::from("player north name"));
        assert!(message_parsing::Direction::North == handler.state.player_direction);
        assert!(message_parsing::Direction::South == handler.state.opponent_direction);
        handler.run_one_round(&ai, String::from("player south name"));
        assert!(message_parsing::Direction::South == handler.state.player_direction);
        assert!(message_parsing::Direction::North == handler.state.opponent_direction);
    }

    #[test]
    fn starter_deck_check() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let colors = vec![String::from("a"),
                          String::from("b"),
                          String::from("c"),
                          String::from("d"),
                          String::from("e"),
                          String::from("f")];
        let colors_vec = vec![mp::Color::Color1,
                              mp::Color::Color2,
                              mp::Color::Color3,
                              mp::Color::Color4,
                              mp::Color::Color5,
                              mp::Color::Color6];
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        assert_eq!(54, handler.state.deck.len());
        assert_eq!(colors, handler.state.colors);
        assert_eq!(colors_vec, handler.state.colors_vec);
    }

    #[test]
    fn claim_status_check() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let claims = vec![mp::ClaimStatus::North, mp::ClaimStatus::South,
                          mp::ClaimStatus::South, mp::ClaimStatus::North,
                          mp::ClaimStatus::Unclaimed, mp::ClaimStatus::Unclaimed,
                          mp::ClaimStatus::North, mp::ClaimStatus::South,
                          mp::ClaimStatus::North,
                          ];
        handler.run_one_round(&ai,
                              String::from("flag claim-status north south south north unclaimed \
                                            unclaimed north south north"));
        assert_eq!(9, handler.state.claim_status.len());
        assert_eq!(claims, handler.state.claim_status);
    }

    #[test]
    fn sides_check() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let mut expected: Vec<Vec<mp::Card>> = Vec::with_capacity(9);
        for _ in 1..10 {
            expected.push(vec![]);
        }
        expected[0] = vec![mp::Card {
                               color: mp::Color::Color1,
                               number: 3,
                           },
                           mp::Card {
                               color: mp::Color::Color2,
                               number: 7,
                           }];
        handler.run_one_round(&ai, String::from("colors red blue 3 4 5 6"));
        handler.run_one_round(&ai, String::from("player north name"));
        handler.run_one_round(&ai, String::from("flag 1 cards north red,3 blue,7"));
        assert_eq!(expected, handler.state.player_side);
    }

    #[test]
    fn sides_check_1() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let mut expected: Vec<Vec<mp::Card>> = Vec::with_capacity(9);
        for _ in 1..10 {
            expected.push(vec![]);
        }
        expected[1] = vec![mp::Card {
                               color: mp::Color::Color1,
                               number: 3,
                           },
                           mp::Card {
                               color: mp::Color::Color2,
                               number: 7,
                           }];
        handler.run_one_round(&ai, String::from("colors red blue 3 4 5 6"));
        handler.run_one_round(&ai, String::from("player north name"));
        handler.run_one_round(&ai, String::from("flag 2 cards south red,3 blue,7"));
        assert_eq!(expected, handler.state.opponent_side);
    }

    #[test]
    fn remove_card_when_opponent_played() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let expected_card = mp::Card {
            color: mp::Color::Color1,
            number: 7,
        };
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        assert!(handler.state.deck.contains(&expected_card));
        handler.run_one_round(&ai, String::from("opponent play 1 a,7"));
        assert_eq!(53, handler.state.deck.len());
        assert!(!handler.state.deck.contains(&expected_card));
    }

    #[test]
    fn remove_card_when_given_player_hand() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let expected_cards = vec![mp::Card {
                                      color: mp::Color::Color1,
                                      number: 7,
                                  },
                                  mp::Card {
                                      color: mp::Color::Color3,
                                      number: 3,
                                  }];
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        handler.run_one_round(&ai, String::from("player north name"));
        for x in &expected_cards {
            assert!(handler.state.deck.contains(&x));
        }
        handler.run_one_round(&ai, String::from("player north hand a,7 c,3"));
        assert_eq!(52, handler.state.deck.len());
        for x in &expected_cards {
            assert!(!handler.state.deck.contains(&x));
        }
    }

    #[test]
    fn hand_update_when_given_player_hand() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let expected_cards = vec![mp::Card {
                                      color: mp::Color::Color1,
                                      number: 7,
                                  },
                                  mp::Card {
                                      color: mp::Color::Color3,
                                      number: 3,
                                  }];
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        handler.run_one_round(&ai, String::from("player north name"));
        handler.run_one_round(&ai, String::from("player north hand a,7 c,3"));
        assert_eq!(expected_cards, handler.state.player_hand);
    }

    #[test]
    fn remove_from_deck_when_flag_status() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let expected_cards = vec![mp::Card {
                                      color: mp::Color::Color1,
                                      number: 7,
                                  },
                                  mp::Card {
                                      color: mp::Color::Color3,
                                      number: 3,
                                  }];
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        handler.run_one_round(&ai, String::from("player north name"));
        for x in &expected_cards {
            assert!(handler.state.deck.contains(&x));
        }
        handler.run_one_round(&ai, String::from("flag 1 cards north a,7 c,3"));
        assert_eq!(52, handler.state.deck.len());
        for x in &expected_cards {
            assert!(!handler.state.deck.contains(&x));
        }
    }

    #[test]
    fn default_vectors_after_certain_commands() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        assert_eq!(54, handler.state.deck.len());
        assert_eq!(6, handler.state.colors.len());
        assert_eq!(9, handler.state.claim_status.len());
        assert_eq!(9, handler.state.opponent_side.len());
        assert_eq!(9, handler.state.player_side.len());
    }
}
