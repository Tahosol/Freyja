use rusqlite::{Connection, Result};
use std::error::Error;

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

        let command = modules::command::check(question);

        if command.0 {
            return Ok(command.1);
        }

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

use std::fs;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub fn litsen() -> String {
    let model = "models/voice-input-english-74.bin";
    return real_time_transcribe(model);
}

fn whisper_run(samples: &[i16], ctx: &WhisperContext) -> String {
    let language = "en";
    let mut state = ctx.create_state().expect("failed to create state");
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    params.set_language(Some(&language));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    let mut inter_samples = vec![Default::default(); samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
        .expect("failed to convert audio data");

    state
        .full(params, &inter_samples[..])
        .expect("failed to run model");

    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");

    let mut transcript = String::new();
    for i in 0..num_segments {
        transcript.push_str(
            &state
                .full_get_segment_text(i)
                .expect("failed to get segment"),
        );
    }
    transcript
}

fn real_time_transcribe(model_path: &str) -> String {
    let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
        .expect("failed to load model");

    let cache = dirs::cache_dir().unwrap().join("Freyja");
    let _ = fs::create_dir_all(&cache);

    loop {
        let recording = Command::new("rec")
            .arg("chunk.wav")
            .arg("channels")
            .arg("1")
            .arg("rate")
            .arg("16000")
            .arg("silence")
            .arg("1")
            .arg("0.1")
            .arg("3%")
            .arg("1")
            .arg("1.0")
            .arg("3%")
            .current_dir(&cache)
            .status()
            .expect("Failed to record audio");

        if recording.success() {
            let samples: Vec<i16> = hound::WavReader::open(cache.join("chunk.wav"))
                .unwrap()
                .into_samples::<i16>()
                .map(|x| x.unwrap())
                .collect();

            let transcript = whisper_run(&samples, &ctx);
            if !transcript.trim().is_empty() {
                return transcript;
            }
        }
    }
}
