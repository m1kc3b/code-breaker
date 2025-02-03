use iocraft::prelude::*;
use std::io::stdin;


fn clear_screen() {
  print!("\x1B[2J\x1B[H");
}

pub fn home_screen() {
  // Clear the screen
  clear_screen();

  // Display home screen
  element! {
      View(
          flex_direction: FlexDirection::Column,
          border_style: BorderStyle::Bold,
          border_color: Color::AnsiValue(93),
          padding: 2,
          margin: 1,
      ) {
          View(
              flex_direction: FlexDirection::Column,
              align_items: AlignItems::Center,
          ) {
              View {
                  Text(content: "Welcome to Code Breaker!", weight: Weight::Bold, color: Color::AnsiValue(93))
              }
              Text(content: format!("The rules of this game are simple: Guessing the code as fast as possible!"))
              Text(content: format!(""))
              Text(content: format!("Menu: (h->help, q->quit)"))
              Text(content: format!(""))
          }
      }
  }
  .print();

}


pub fn prompt(content: &str) {
  element! {
    View() {
        View{
            Text(content, weight: Weight::Bold, color: Color::AnsiValue(93))
            Text(content: "or press 'h' (help) or 'q' (quit)", color: Color::DarkGrey)
        }
    }
  }.print();
}

pub fn info(content: &str) {
  element! {
    View() {
        View{
            Text(content)
        }
    }
  }.print();
}

pub fn read_input() -> String {
  let mut buffer = String::new();
  stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

pub fn help() {
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
            Text(content: "In medium mode, you have 6 digits to guess instead of 4.")
            Text(content: "In hard mode, you have 8 digits to guess and the clues no longer indicate the position of the good digits.")
            Text(content: "Ready? Let's play!")
    }
  }.print();
}

pub fn error(content: &str) {
  element! {
      View(
          flex_direction: FlexDirection::Column,
          border_style: BorderStyle::Single, 
          border_color: Color::Red,
          margin: 1,
          padding:1
      ) {
              Text(content, color: Color::Red)
      }
  }.print();
}

pub fn success(content: &str) {
  element! {
      View(
          flex_direction: FlexDirection::Column,
          border_style: BorderStyle::Single, 
          border_color: Color::AnsiValue(154),
          margin: 1,
          padding:1
      ) {
          Text(content, color: Color::AnsiValue(154))
      }
  }.print();
}

pub fn quit(secret: &Vec<u8>, attempts: u8) {
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
}