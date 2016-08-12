use message_parsing;

pub trait AiInterface {
    fn update_game_state(&self, state: &GameState) -> String;
    fn get_bot_name(&self) -> String;
}

#[derive(Default)]
pub struct GameState {
    pub player_direction: message_parsing::Direction,
    pub opponent_direction: message_parsing::Direction,
    deck: Vec<message_parsing::Card>,
    colors: Vec<String>,
    claim_status: Vec<message_parsing::ClaimStatus>,
    opponent_side: Vec<Vec<message_parsing::Card>>,
    player_side: Vec<Vec<message_parsing::Card>>,
    pub player_hand: Vec<message_parsing::Card>,
}

#[derive(Default)]
pub struct GameHandler {
    state: GameState,
}


impl GameHandler {
    pub fn run_one_round(&mut self, ai: &AiInterface, message: String) {
        let x = message_parsing::parse_message(message);
        match x {
            message_parsing::Message::OpponentPlay { number: number, card: card } => {
                if let Some(index) = self.state.deck.iter().position(|i| *i == card) {
                    self.state.deck.remove(index);
                }
            }
            message_parsing::Message::PlayerHand { direction: direction, cards: cards } => {
                for card in &cards {
                    if let Some(index) = self.state.deck.iter().position(|i| *i == *card) {
                        self.state.deck.remove(index);
                    }
                }
                self.state.player_hand = cards;
            }
            message_parsing::Message::FlagStatus { flag_num: num,
                                                   direction: direction,
                                                   cards: cards } => {

                for card in &cards {
                    if let Some(index) = self.state.deck.iter().position(|i| *i == *card) {
                        self.state.deck.remove(index);
                    }
                }
                if self.state.player_side.len() == 0 || self.state.opponent_side.len() == 0 {
                    for i in 1..10 {
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
            message_parsing::Message::PlayerDirection { direction: direction, .. } => {
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
            message_parsing::Message::ColorNames { colors: colors } => {
                for i in 1..10 {
                    for x in colors.to_vec() {
                        self.state.deck.push(message_parsing::Card {
                            color: x,
                            number: i,
                        });
                    }
                }
                self.state.colors = colors;
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
        fn update_game_state(&self, state: &GameState) -> String {
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
        handler.run_one_round(&ai, String::from("colors a b c d e f"));
        assert_eq!(54, handler.state.deck.len());
        assert_eq!(colors, handler.state.colors);
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
        for i in 1..10 {
            expected.push(vec![]);
        }
        expected[0] = vec![mp::Card {
                               color: String::from("red"),
                               number: 3,
                           },
                           mp::Card {
                               color: String::from("blue"),
                               number: 7,
                           }];
        handler.run_one_round(&ai, String::from("player north name"));
        handler.run_one_round(&ai, String::from("flag 1 cards north red,3 blue,7"));
        assert_eq!(expected, handler.state.player_side);
    }

    #[test]
    fn sides_check_1() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let mut expected: Vec<Vec<mp::Card>> = Vec::with_capacity(9);
        for i in 1..10 {
            expected.push(vec![]);
        }
        expected[1] = vec![mp::Card {
                               color: String::from("red"),
                               number: 3,
                           },
                           mp::Card {
                               color: String::from("blue"),
                               number: 7,
                           }];
        handler.run_one_round(&ai, String::from("player north name"));
        handler.run_one_round(&ai, String::from("flag 2 cards south red,3 blue,7"));
        assert_eq!(expected, handler.state.opponent_side);
    }

    #[test]
    fn remove_card_when_opponent_played() {
        let mut handler: GameHandler = Default::default();
        let ai = TestAi {};
        let colors = vec![String::from("a"),
                          String::from("b"),
                          String::from("c"),
                          String::from("d"),
                          String::from("e"),
                          String::from("f")];
        let expected_card = mp::Card {
            color: String::from("a"),
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
        let colors = vec![String::from("a"),
                          String::from("b"),
                          String::from("c"),
                          String::from("d"),
                          String::from("e"),
                          String::from("f")];
        let expected_cards = vec![mp::Card {
                                      color: String::from("a"),
                                      number: 7,
                                  },
                                  mp::Card {
                                      color: String::from("c"),
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
                                      color: String::from("a"),
                                      number: 7,
                                  },
                                  mp::Card {
                                      color: String::from("c"),
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
                                      color: String::from("a"),
                                      number: 7,
                                  },
                                  mp::Card {
                                      color: String::from("c"),
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
}
