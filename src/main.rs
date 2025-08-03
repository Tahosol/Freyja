mod frey_core;
mod modules;

use colored::Colorize;

#[tokio::main]
async fn main() {
    let model = include_str!("../models/readme.txt").trim();
    let mut freyja = frey_core::Elms::default();
    println!("Freyja: Hi how can I help you?");
    frey_core::talk("Hi how can I help you?");
    loop {
        if freyja.command {
            break;
        }
        let input = frey_core::litsen(model);
        println!("{}{}", "User: ".bold(), input.bold());
        let input = input.trim().to_lowercase();
        if (input.contains("stop") && input.contains("chat")) || input.contains("bye") {
            println!("Freyja: Bye!");
            frey_core::talk("Bye!");
            break;
        }
        let answer = match freyja.get_answer(&input).await {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        println!("{}{}", "Freyja: ".green(), answer.green().bold());
        frey_core::talk(&answer);
    }
}
