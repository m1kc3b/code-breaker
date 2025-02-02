mod clear;

use rand;
use std::io::stdin;
use iocraft::prelude::*;

use clear::clear_screen;

#[allow(unused_variables, dead_code)]
fn main() {
    // Clear the screen
    clear_screen();

    // Numbers of attempts
    let mut attempts = 0;

    // Generate 4 random digits - our 'secret'
    let mut secret: Vec<u8> = Vec::with_capacity(4);
    secret.extend((0..4).map(|_| rand::random_range(0..=9)));

    // Init mainly random color
    let main_color = Color::AnsiValue(rand::random_range(0..=255));
    let orange_color = Color::AnsiValue(208);
    let green_color = Color::AnsiValue(154);

    // Go into a loop
    element! {
        View(
            flex_direction: FlexDirection::Column,
            width: 500pct,
            border_style: BorderStyle::Bold,
            border_color: main_color,
            padding: 2,
            margin: 1,
        ) {
            View(
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            ) {
                View {
                    Text(content: "Welcome to Code Breaker!", weight: Weight::Bold, color: main_color)
                }
                Text(content: format!("Guess the secret code!"))
            }
        }
    }
    .print();

    // TODO:
    // remove this lines
    // println!("==(The secret is: {:?})==", secret);

    
    loop {
        element! {
            View() {
                View{
                    Text(content: "Enter four digits (0-9)", weight: Weight::Bold, color: main_color)
                    Text(content: " or press 'q' to quit", color: Color::DarkGrey)
                }
            }
        }.print();

        // Read a string from Standard In and trim the whitespace off it
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let input = buffer.trim();

        // TODO:
        // handle quitting
        if input == "q" {
            element! {
                View(
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single, 
                    border_color: orange_color,
                    padding: 1,
                    margin: 1,
                ) {
                        Text(content: "Quitting...", color: orange_color, weight: Weight::Bold)
                        Text(content: format!("The secret was {}{}{}{}\n", secret[0], secret[1], secret[2], secret[3]), color: orange_color)
                        Text(content: format!("You've made {} attempts\n", attempts), color: orange_color)
                        Text(content: "Goodbye 👋👋👋", color: orange_color)
                    }
            }.print();
            break;
        }

        // Parse that string into a guess, containing four digits (give an error if the user makes a mistake)
        let guess = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect::<Vec<u8>>();
        
        // handle error cases
        if guess.len() != 4 {
            element! {
                View(
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single, 
                    border_color: Color::Red,
                    margin: 1,
                    padding:1
                ) {
                        Text(content: "You must enter exactly four digits (no chars or symbols)!", color: Color::Red)
                }
            }.print();
            continue;
        }


        // TODO: 
        // remove this lines
        // println!("You entered: {}{}{}{}", guess[0], guess[1], guess[2], guess[3]);

        // Run the calculation routine above and print the coloured blocks
        let result = calc_green_and_yellow(&secret, &guess);
        attempts += 1;

        
        // Exit if all the blocks are green
        if result == "🟩🟩🟩🟩" {

            element! {
                View(
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single, 
                    border_color: green_color,
                    margin: 1,
                    padding:1
                ) {
                    Text(content: format!("Congratulations! You've cracked the code in {} attempts!", attempts), color: green_color)
                }
            }.print();
            break;
        } else {
            element! {
                View(
                    flex_direction: FlexDirection::Column,
                    padding:1
                ) {
                    View{
                        Text(content: format!("#{} -> {}", attempts, result))
                    }
                }
            }.print();
        }
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