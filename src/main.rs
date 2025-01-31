
fn main() {
    // TODO:
    // Generate 4 random digits - our 'secret'
    // Go into a loop
    // Read a string from Standard In and trim the whitespace off it
    // Parse that string into a guess, containing four digits (give an error if the user makes a mistake)
    // Run the calculation routine above and print the coloured blocks
    // Exit if all the blocks are green
}

fn calc_green_and_yellow(secret: &[u8], guess: &[u8]) -> String {
    todo!()
}


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
        "🟥🟥🟨🟥"
    );
}

#[test]
fn two_in_secret_one_in_guess() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
        "🟥🟥🟨🟥"
    );
}
