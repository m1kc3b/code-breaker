/// Generate a random secret number of a given size
pub fn generate_secret(size: usize) -> Vec<u8> {
  use rand;
  let mut secret: Vec<u8> = Vec::with_capacity(size);
  secret.extend((0..size).map(|_| rand::random_range(0..=9)));
  secret
}

/// Check the guess against the secret number and return the result
fn check_guess(secret: &[u8], guess: &[u8]) -> String {
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