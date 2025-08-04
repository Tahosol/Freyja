mod frey_core;
mod modules;

use colored::Colorize;

use crate::modules::config;
#[tokio::main]
async fn main() {
    let config = config::get();
    let mut freyja = frey_core::Elms::default();
    println!("Freyja: Hi how can I help you?");
    frey_core::talk("Hi how can I help you?");
    loop {
        if freyja.command {
            break;
        }
        let input = frey_core::litsen(&config.whisper_model_path);
        println!("{}{}", "User: ".bold(), input.bold());
        let input = input.trim().to_lowercase();
        if (input.contains("stop") && input.contains("chat")) || input.contains("bye") {
            println!("Freyja: Bye!");
            frey_core::talk("Bye!");
            break;
        }
        let answer = match freyja
            .get_answer(
                &input,
                &config.ollama_host,
                config.ollama_port,
                &config.ollama_model,
            )
            .await
        {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        println!("{}{}", "Freyja: ".green(), answer.green().bold());

        let talk_handle = tokio::spawn({
            async move {
                frey_core::talk(&answer);
            }
        });

        let stdin_handle = tokio::spawn(async {
            let mut user_input = String::new();
            if std::io::stdin().read_line(&mut user_input).is_ok() {
                if user_input == "\n" {
                    return true;
                }
            }
            false
        });

        tokio::select! {
            _ = talk_handle => {
            },
            should_skip = stdin_handle => {
                if should_skip.unwrap_or(false) {
                    continue;
                }
            }
        }
    }
}
