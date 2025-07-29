use std::error::Error;

pub struct Elms {
    pub command: bool,
}

impl Default for Elms {
    fn default() -> Self {
        Self { command: false }
    }
}

use crate::modules;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

impl Elms {
    pub async fn get_answer(&mut self, question: &str) -> Result<String, Box<dyn Error>> {
        let command = modules::command::check(question);

        if command.0 {
            self.command = true;
            return Ok(command.1);
        }

        if modules::weather::detect_weather_ask(question) {
            let weather = modules::weather::get();
            return Ok(weather);
        }
        let ollama = Ollama::default();
        let model = "freyja_gama".to_string();
        let res = ollama
            .generate(GenerationRequest::new(model, question))
            .await;

        if let Ok(res) = res {
            return Ok(res.response);
        } else {
            return Ok("Silent..".to_string());
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

pub fn litsen(model: &str) -> String {
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
        let _ = modules::volume_control::mute();
        let recording = Command::new("rec")
            .arg("chunk.wav")
            .arg("channels")
            .arg("1")
            .arg("rate")
            .arg("16000")
            .arg("silence")
            .arg("1")
            .arg("0.1")
            .arg("3.3%")
            .arg("1")
            .arg("1.0")
            .arg("3.3%")
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
                let _ = modules::volume_control::unmute();
                return transcript;
            }
        }
    }
}
