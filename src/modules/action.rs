use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Action {
    in_terminal: bool,
    terminal: Option<String>,
    sudo: bool,
    main_command: String,
    args: Vec<String>,
    key_word: Vec<String>,
    return_message: String,
}

use std::{error::Error, fs};

pub fn check(que: &str) -> (bool, String) {
    match collect_actions() {
        Ok(o) => {
            for require in o {
                if contains_keywords(que, &require.key_word) {
                    let act = take_action(
                        &require.main_command,
                        &require.args,
                        require.sudo,
                        require.in_terminal,
                        require.terminal,
                    );
                    if act.is_ok() {
                        return (true, require.return_message);
                    }
                }
            }
            (false, String::new())
        }
        Err(_e) => (false, String::new()),
    }
}

fn contains_keywords(text: &str, keywords: &Vec<String>) -> bool {
    keywords
        .iter()
        .all(|keyword| text.contains(&keyword.to_lowercase()))
}

use std::process::Command;

fn take_action(
    command: &str,
    args: &Vec<String>,
    sudo: bool,
    terminal_on: bool,
    terminal: Option<String>,
) -> Result<(), Box<dyn Error>> {
    if sudo && terminal_on {
        let term = terminal.expect("Fail to read terminal name in the config file");
        let mut action = Command::new("setsid");
        action.arg(&term);
        action.arg("--hold");
        action.arg("sudo");
        action.arg(command);
        for a in args {
            action.arg(a);
        }
        action.spawn()?;
    } else if sudo {
        let mut action = Command::new("setsid");
        action.arg("sudo");
        action.arg(command);
        for a in args {
            action.arg(a);
        }
        action.spawn()?;
    } else if terminal_on {
        let term = terminal.expect("Fail to read terminal name in the config file");
        let mut action = Command::new("setsid");
        action.arg(&term);
        action.arg("--hold");
        action.arg(command);
        for a in args {
            action.arg(a);
        }
        action.spawn()?;
    } else {
        let mut action = Command::new("setsid");
        action.arg(command);
        for a in args {
            action.arg(a);
        }
        action.spawn()?;
    }
    Ok(())
}

fn collect_actions() -> Result<Vec<Action>, Box<dyn Error>> {
    let config = dirs::config_dir()
        .expect("Fail to find config path")
        .join("freyja")
        .join("actions");
    fs::create_dir_all(&config)?;
    let ac: Vec<_> = fs::read_dir(config)?
        .filter_map(|x| {
            let entry = x.ok()?;
            if entry.path().extension().and_then(|x| x.to_str()) == Some("toml") {
                let string = fs::read_to_string(entry.path()).ok()?;
                let action: Action = toml::from_str(&string).ok()?;
                Some(action)
            } else {
                None
            }
        })
        .collect();
    Ok(ac)
}
