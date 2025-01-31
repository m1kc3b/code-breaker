
use rand;
use std::io::stdin;
use iocraft::prelude::*;


#[allow(unused_variables, dead_code)]
fn main() {
    // Numbers of attempts
    let mut attempts = 0;
    // Generate 4 random digits - our 'secret'
    let mut secret: Vec<u8> = Vec::with_capacity(4);
    secret.extend((0..4).map(|_| rand::random_range(0..=9)));

    // Go into a loop
    element! {
        View(
            flex_direction: FlexDirection::Column,
            width: 500pct,
            padding: 2,
        ) {
            View(
                flex_direction: FlexDirection::Column,
                border_style: BorderStyle::Single,
                border_color: Color::DarkGrey,
                border_edges: Edges::Bottom,
                align_items: AlignItems::Center,
                width: 100pct,
            ) {
                View {
                    Text(content: "Welcome to Code Breaker!", weight: Weight::Bold)
                }
                Text(content: format!("Guess the secret code!"), color: Color::DarkGrey)
            }
        }
    }
    .print();

    // TODO:
    // remove this lines
    println!("==(The secret is: {:?})==", secret);

    
    loop {
        element! {
            View() {
                View{
                    Text(content: "Enter four digits (1-9)", weight: Weight::Bold)
                    Text(content: " or press 'q' to quit", color: Color::DarkGrey)
                }
            }
        }.print();

        // Read a string from Standard In and trim the whitespace off it
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        // Parse that string into a guess, containing four digits (give an error if the user makes a mistake)
        let guess = buffer
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect::<Vec<u8>>();
        
        // handle error cases
        if guess.len() != 4 {
            println!("You must enter exactly four digits (no chars or symbols)!");
            continue;
        }

        // TODO:
        // handle quitting

        // TODO: 
        // remove this lines
        println!("You entered: {:?}", guess);

        // Run the calculation routine above and print the coloured blocks
        let result = calc_green_and_yellow(&secret, &guess);
        println!("{}", result);
        println!("Attempts: {}", attempts);
        
        // Exit if all the blocks are green
        if result == "🟩🟩🟩🟩" {
            println!("Congratulations! You've cracked the code in {} attempts!", attempts);
            break;
        } else {
            
        }
        attempts += 1;
    }
    
}

#[allow(unused_variables, dead_code)]
fn calc_green_and_yellow(secret: &[u8], guess: &[u8]) -> String {
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
    fn all_wrong() {
        assert_eq!(
            &calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
            "🟥🟥🟥🟥"
        );
    }
    
    #[test]
    fn all_green() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
            "🟩🟩🟩🟩"
        );
    }
    
    #[test]
    fn one_wrong() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
            "🟩🟩🟩🟥"
        );
    }
    
    #[test]
    fn all_yellow() {
        assert_eq!(
            &calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
            "🟨🟨🟨🟨"
        );
    }
    
    #[test]
    fn one_wrong_but_duplicate() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
            "🟩🟩🟩🟥"
        );
    }
    
    #[test]
    fn one_right_others_duplicate() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
            "🟩🟥🟥🟥"
        );
    }
    
    #[test]
    fn two_right_two_swapped() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
            "🟨🟩🟩🟨"
        );
    }
    
    #[test]
    fn two_wrong_two_swapped() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
            "🟨🟥🟥🟨"
        );
    }
    
    #[test]
    fn a_bit_of_everything() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
            "🟩🟥🟨🟨"
        );
    }
    
    #[test]
    fn two_in_guess_one_in_secret() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 3], &[3, 9, 9, 9]),
            "🟨🟥🟥🟥"
        );
    }
    
    #[test]
    fn two_in_secret_one_in_guess() {
        assert_eq!(
            &calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
            "🟨🟥🟥🟥"
        );
    }
    
}