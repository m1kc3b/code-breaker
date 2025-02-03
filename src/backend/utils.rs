use std::io::stdin;

/// Generate a random secret number of a given size
pub fn generate_secret(size: usize) -> Vec<u8> {
  use rand;
  let mut secret: Vec<u8> = Vec::with_capacity(size);
  secret.extend((0..size).map(|_| rand::random_range(0..=9)));
  secret
}

/// Read the input from the user
pub fn read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn parse_input(input: &str) -> Vec<u8> {
  input.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect()
}

/// Check the guess against the secret number and return the result
pub fn check_guess(secret: &[u8], guess: &[u8]) -> String {
    
    // TODO: Handle the difficulty of the Game::Level
    // if self.level == GameLevel::Hard
    // random the result

  let mut result = vec!['🟥'; secret.len()];
  let mut secret_counts = [0; 10];
  let mut guess_counts = [0; 10];

  // check for Green case
  for (i, &digit) in guess.iter().enumerate() {
      if secret[i] == digit {
          result[i] = '🟩';
          
      } else {
          secret_counts[secret[i] as usize] += 1;
      }
  }

  // check for Yellow case
  for (i, &digit) in guess.iter().enumerate() {
      if result[i] == '🟩' {
          continue;
      }
      if secret_counts[digit as usize] > guess_counts[digit as usize] {
          result[i] = '🟨';
          guess_counts[digit as usize] += 1;
      }
  }
      

  result.iter().collect::<String>()
}



mod test {
  #[allow(unused_imports)]
  use super::*;

  #[test]
  fn parse_input_test() {
      assert_eq!(
          &parse_input("1234"),
          &[1, 2, 3, 4]
      );
  }

  #[test]
  fn all_wrong() {
      assert_eq!(
          &check_guess(&[5, 6, 7, 8], &[1, 2, 3, 4]),
          "🟥🟥🟥🟥"
      );
  }
  
  #[test]
  fn all_green() {
      assert_eq!(
          &check_guess(&[1, 2, 3, 4], &[1, 2, 3, 4]),
          "🟩🟩🟩🟩"
      );
  }
  
  #[test]
  fn one_wrong() {
      assert_eq!(
          &check_guess(&[1, 2, 3, 5], &[1, 2, 3, 4]),
          "🟩🟩🟩🟥"
      );
  }
  
  #[test]
  fn all_yellow() {
      assert_eq!(
          &check_guess(&[4, 3, 2, 1], &[1, 2, 3, 4]),
          "🟨🟨🟨🟨"
      );
  }
  
  #[test]
  fn one_wrong_but_duplicate() {
      assert_eq!(
          &check_guess(&[1, 2, 3, 1], &[1, 2, 3, 4]),
          "🟩🟩🟩🟥"
      );
  }
  
  #[test]
  fn one_right_others_duplicate() {
      assert_eq!(
          &check_guess(&[1, 1, 1, 1], &[1, 2, 3, 4]),
          "🟩🟥🟥🟥"
      );
  }
  
  #[test]
  fn two_right_two_swapped() {
      assert_eq!(
          &check_guess(&[1, 2, 2, 2], &[2, 2, 2, 1]),
          "🟨🟩🟩🟨"
      );
  }
  
  #[test]
  fn two_wrong_two_swapped() {
      assert_eq!(
          &check_guess(&[1, 3, 3, 2], &[2, 2, 2, 1]),
          "🟨🟥🟥🟨"
      );
  }
  
  #[test]
  fn a_bit_of_everything() {
      assert_eq!(
          &check_guess(&[1, 9, 4, 3], &[1, 2, 3, 4]),
          "🟩🟥🟨🟨"
      );
  }
  
  #[test]
  fn two_in_guess_one_in_secret() {
      assert_eq!(
          &check_guess(&[1, 2, 3, 3], &[3, 9, 9, 9]),
          "🟨🟥🟥🟥"
      );
  }
  
  #[test]
  fn two_in_secret_one_in_guess() {
      assert_eq!(
          &check_guess(&[1, 2, 3, 4], &[3, 3, 9, 9]),
          "🟨🟥🟥🟥"
      );
  }
  
}