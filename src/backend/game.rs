use super::display::*;
use super::utils::{check_guess, generate_secret, parse_input, read_input};
use super::results::GameResult;
use iocraft::prelude::*;
use std::process::exit;

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
            game.help();
            continue;
          },
          "q" => {
            println!("Goodbye 👋👋👋");
            exit(0);
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
              self.help();
              continue;
            },
            "q" => {
              self.quit();
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
              let result = check_guess(&self.secret, &guess, level);

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

    fn help(&self) {
      element! {
        View(flex_direction: FlexDirection::Column) {
                Text(content: "The rules of this game are simple: Guessing the code as fast as possible!")
                Text(content: "In easy mode, you have 4 digits to guess.")
                Text(content: "After each attempt we will provide you a clue:")
                Text(content: "- 🟩 means good digit and at the right place.")
                Text(content: "- 🟨 means code contains this digit.")
                Text(content: "- 🟥 means code doesn't contain this digit.")
                Text(content: "For example, 🟩🟥🟨🟨 means you've found out 3 of 4 digits but only one is at the good place.")
                Text(content: "")
                Text(content: "In medium mode, you have 5 digits to guess instead of 4.")
                Text(content: "In hard mode, you have 6 digits to guess and the clues no longer indicate the position of the good digits.")
                Text(content: "Ready? Let's play!")
        }
      }.print();
    }
    
    fn quit(&self) {
      let secret = &self.secret;
      let attempts = self.result.attempts;
      element! {
          View(
              flex_direction: FlexDirection::Column,
              border_style: BorderStyle::Single, 
              border_color: Color::AnsiValue(208),
              padding: 1,
              margin: 1,
          ) {
                  Text(content: "Quitting...", color: Color::AnsiValue(154), weight: Weight::Bold)
                  Text(content: format!("The secret was {}{}{}{}\n", secret[0], secret[1], secret[2], secret[3]), color: Color::AnsiValue(208))
                  Text(content: format!("You've made {} attempts\n", attempts), color: Color::AnsiValue(208))
                  Text(content: "Goodbye 👋👋👋", color: Color::AnsiValue(208))
              }
      }.print();
      // Exit
      exit(1);
    }

  }
