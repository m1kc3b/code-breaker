use iocraft::prelude::*;


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