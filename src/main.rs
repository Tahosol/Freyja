mod frey_core;
mod modules;

use std::io;
fn main() {
    // let _ = elms::train_in("generated");
    let elms_bot = frey_core::Elms::default();
    println!("Chat with Elms:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();
        if input == "stop the chat" {
            println!("Elms: Bye!");
            frey_core::talk("Bye!");
            break;
        }
        let answer = match elms_bot.get_answer(&input) {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        println!("Elms: {answer}");
        frey_core::talk(&answer);
    }
}
