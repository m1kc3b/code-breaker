use super::display::*;
use super::utils::{check_guess, generate_secret, parse_input, read_input};
use super::results::GameResult;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameLevel {
    Easy = 4,
    Medium = 5,   
    Hard = 6,
}

pub struct Game {
  level: Option<GameLevel>,
  secret: Vec<u8>,
  result: GameResult,
}

impl Game {
    fn default() -> Self {
        Game {
            level: None,
            secret: Vec::new(),
            result: GameResult::new(),
        }
    }

    fn update_level(&mut self, level: GameLevel) {
        self.level = Some(level);
        self.secret = generate_secret(level as usize);
    }

    pub fn start() {
      // Display the welcome message
      home_screen();
      let mut game = Game::default();

      loop {
        // Ask for the game level
        prompt("Choose your mode: '1' (easy), '2' (medium), '3' (hard) ");
  
        // Read the input
        match read_input().as_str() {
          "1" => {
            game.update_level(GameLevel::Easy);
            game.play();
            continue;
          },
          "2" => {
            game.update_level(GameLevel::Medium);
            game.play();
            continue;
          },
          "3" => {
            game.update_level(GameLevel::Hard);
            game.play();
            continue;
          },
          "h" => {
            help();
            continue;
          },
          "q" => {
            quit(&game.secret, &game.result.attempts);
            break;
          },
          _ => {
            error("Unknow command!");
            continue;
          },
            
        }
      }

    }

    fn play(&mut self) {

      let level = self.level.unwrap() as u8;

      info(format!("You have to guess {} digits", level).as_str());
      loop {
          // Display prompt
          prompt("Enter your guess: ");
          let input = read_input();
          // Match the input
          match input.as_str() {
            "h" => {
              help();
              continue;
            },
            "q" => {
              quit(&self.secret, &self.result.attempts);
              break;
            },
            _ => {
              let guess = parse_input(input.as_str());

              // handle error cases
              if guess.len() != level as usize {
                error(format!("You must enter exactly {} digits (no chars or symbols)!", level).as_str());
                continue;
              }

              // Check the guess
              let result = check_guess(&self.secret, &guess, &level);

              // Increment the attempts
              self.result.attempts += 1;

              // Display the result
              if result.chars().all(|c| c == '🟩') {
                success(format!("Congratulations! You've cracked to the code in {} attemps", self.result.attempts).as_str());
                // TODO: Store the result
                // self.result.store().unwrap();
                break;

                // TODO: Ask to play again
              } else {
                info(format!("#{} -> {}", self.result.attempts, result).as_str());
                continue;
              }
            }
          }
      }
    }

  }
