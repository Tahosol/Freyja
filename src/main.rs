mod frey_core;
mod modules;

use colored::Colorize;

fn main() {
    // let data_folder = vec!["english", "french", "generated"];
    // let _ = modules::data::train(data_folder);
    let freyja = frey_core::Elms::default();
    println!("Freyja: Hi how can I help you?");
    frey_core::talk("Hi how can I help you?");
    loop {
        // let mut input = String::new();
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read line");
        let input = frey_core::litsen();
        println!("{}{}", "User: ".bold(), input.bold());
        let input = input.trim().to_lowercase();
        if input.contains("stop") && input.contains("chat") {
            println!("Freyja: Bye!");
            frey_core::talk("Bye!");
            break;
        }
        let answer = match freyja.get_answer(&input) {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        println!("{}{}", "Freyja: ".green(), answer.green().bold());
        frey_core::talk(&answer);
    }
}
