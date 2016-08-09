use message_parsing;
use std::io;

pub trait AiInterface {
    fn update_game_state(&self, state: &GameState) -> String;
    fn get_bot_name(&self) -> String;
}

#[derive(Default)]
pub struct GameState {
}

#[derive(Default)]
pub struct GameHandler {
    state: GameState,
}


impl GameHandler {
    fn run_one_round(&self, ai: &AiInterface) {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("failed to read line");

        let x = message_parsing::parse_message(message);
        match x {
            message_parsing::Message::PlayerDirection { direction: direction, .. } => {
                println!("player {} {}",
                         message_parsing::get_direction_string(direction),
                         ai.get_bot_name());
            }
            message_parsing::Message::PlayCard => {
                println!("{}", ai.update_game_state(&self.state));
            }
            _ => {}
        }
    }
}
