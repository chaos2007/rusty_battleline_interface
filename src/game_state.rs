use message_parsing;

pub trait AiInterface {
    fn update_game_state(&self, state: &GameState) -> String;
    fn get_bot_name(&self) -> String;
}

#[derive(Default)]
pub struct GameState {
    player_direction: message_parsing::Direction,
    opponent_direction: message_parsing::Direction,
    deck: Vec<message_parsing::Card>,
    colors: Vec<String>,
}

#[derive(Default)]
pub struct GameHandler {
    state: GameState,
}


impl GameHandler {
    pub fn run_one_round(&mut self, ai: &AiInterface, message: String) {
        let x = message_parsing::parse_message(message);
        match x {
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
}
