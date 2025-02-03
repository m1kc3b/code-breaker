/// Generate a random secret number of a given size
pub fn generate_secret(size: usize) -> Vec<u8> {
  use rand;
  let mut secret: Vec<u8> = Vec::with_capacity(size);
  secret.extend((0..size).map(|_| rand::random_range(0..=9)));
  secret
}