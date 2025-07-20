use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn read_data(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let dir = PathBuf::from(path);
    let read_dir = fs::read_dir(dir)?;

    let mut data_list = vec![];

    for item in read_dir.into_iter() {
        let item = item?;
        if item.path().is_file() {
            data_list.push(fs::read_to_string(item.path())?);
        }
    }

    Ok(data_list)
}

use rusqlite::{Connection, Result};

#[allow(dead_code)]
pub fn train_in(path: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("db.sqlite")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversation (
                id   INTEGER PRIMARY KEY,
                que  TEXT NOT NULL,
                ans  TEXT NOT NULL
            )",
        (),
    )?;
    let data = match read_data(path) {
        Ok(data) => data,
        Err(e) => {
            println!("{e}");
            Vec::new()
        }
    };
    for file in data {
        let math: Data = match serde_yaml::from_str(&file) {
            Ok(d) => d,
            Err(e) => {
                println!("Failed to parse YAML: {e}");
                continue;
            }
        };
        for item in math.conversations {
            conn.execute(
                "INSERT INTO conversation (que, ans) VALUES (?1, ?2)",
                (&item[0], &item[1]),
            )?;
        }
    }
    Ok(())
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    #[allow(dead_code)]
    conversations: Vec<Vec<String>>,
}

use bk_tree::{BKTree, metrics};

pub struct Elms {
    data_connection: Connection,
    memory: BKTree<String>,
}

impl Default for Elms {
    fn default() -> Self {
        let conn = match Connection::open("db.sqlite") {
            Ok(con) => con,
            Err(e) => {
                println!("{e}");
                Connection::open_in_memory().unwrap()
            }
        };

        let mut tree: BKTree<String> = BKTree::new(metrics::Levenshtein);

        {
            let mut stmt = conn.prepare("SELECT que FROM conversation").unwrap();
            let que_iter = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();

            for que in que_iter {
                match que {
                    Ok(q) => tree.add(q),
                    Err(_) => {}
                }
            }
        }

        Self {
            data_connection: conn,
            memory: tree,
        }
    }
}

use crate::modules;
use rand::random_range;
impl Elms {
    pub fn get_answer(&self, question: &str) -> Result<String, Box<dyn Error>> {
        let error_answer = modules::default_answer::get();
        let random = random_range(0..error_answer.len());
        let error_code = error_answer[random].clone();

        if modules::weather::detect_weather_ask(question) {
            let weather = modules::weather::get();
            return Ok(weather);
        }
        let answer = self.memory.find(question, 5).collect::<Vec<_>>();
        if let Some((_, text)) = answer.iter().min_by_key(|&(num, _)| *num).cloned() {
            let mut stmt = self
                .data_connection
                .prepare("SELECT ans FROM conversation WHERE que = ?")?;
            let answers_iter = stmt.query_map([text], |row| row.get::<_, String>(0))?;
            let mut answers = Vec::new();
            for answer in answers_iter {
                answers.push(answer?);
            }
            if !answers.is_empty() {
                let random = random_range(0..answers.len());
                if let Some(selected) = answers.get(random) {
                    return Ok(selected.clone());
                } else {
                    return Err(error_code.into());
                }
            } else {
                return Err(error_code.into());
            }
        } else {
            return Err(error_code.into());
        }
    }
}
use std::process::Command;
pub fn talk(text: &str) {
    let dispatcher = Command::new("spd-say").arg("-w").arg(text).output();
    if let Some(e) = dispatcher.err() {
        eprint!("{e}")
    }
}
