use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn read(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
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

#[derive(Debug, Deserialize)]
struct Data {
    #[allow(dead_code)]
    conversations: Vec<Vec<String>>,
}

#[allow(dead_code)]
pub fn train(path: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("db.sqlite")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversation (
                id   INTEGER PRIMARY KEY,
                que  TEXT NOT NULL,
                ans  TEXT NOT NULL
            )",
        (),
    )?;
    for p in path {
        let data = match read(p) {
            Ok(data) => data,
            Err(e) => {
                println!("{e}");
                Vec::new()
            }
        };
        for data_item in data {
            let math: Data = match serde_yaml::from_str(&data_item) {
                Ok(d) => d,
                Err(e) => {
                    println!("Failed to parse YAML: {e}");
                    continue;
                }
            };
            for item in math.conversations {
                for index in 1..item.len() {
                    conn.execute(
                        "INSERT INTO conversation (que, ans) VALUES (?1, ?2)",
                        (&item[0], &item[index]),
                    )?;
                }
            }
        }
    }
    Ok(())
}
