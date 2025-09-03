# Freyja

Freyja is an AI assistant that works locally and can perform actions (i.e., commands). Even though it is still highly experimental, Freyja should now work reasonably well on any Linux computer if it is configured properly.

# How to Install?

As mentioned previously, the project is highly experimental, so the installation process *WILL* be tedious.

## Download Dependencies

Here is a brief list of dependencies:

* [SoX](https://pkgs.org/search/?q=SoX) for recording
* [speech-dispatcher](https://pkgs.org/search/?q=speech-dispatcher) (you won't get the voice exactly like mine if you don't use the Piper Amy voice) for TTS
* [Rust](https://rustup.rs/) (your distro's native package will probably work too) - I think this is a Rust project
* A Whispercpp recognition model (I recommend using [this one](https://keyboard.futo.org/voice-input-models) from FUTO) for STT
  * Larger model = slower speed and better quality (if you have a broken English accent like mine, then use the largest one - I think it's worth it)
* [Ollama](https://pkgs.org/search/?q=ollama) (also pull a model from Ollama) for LLM
  * Potato PC: [Smoll2](https://ollama.com/library/smollm2)
  * Average PC: [llama3.2](https://ollama.com/library/llama3.2)
  * The LLM mastermind: Do whatever you want :)
* Optional (if you want to enable wake-on-call):
  * [Vosk model](https://alphacephei.com/vosk/models) for wake-on-call detection
  * Python


## Clone This Repo
```
git clone --recurse-submodules -j8 https://github.com/Tahosol/Freyja.git
cd Freyja
```

Now, if you list the repository, you will see something like this:

```
 actions   Cargo.lock   Cargo.toml   models   README.md  󱧼 src   target   wakeup-for-freyja
```

## Compile(Fun time)
In the Freyja root directory:
```
cargo build --release
```

Optional (If you want to enable wake-on-call):

* Change into the `wake-up-for-freyja` directory
* Follow the tutorial in the `README` file of `wake-up-for-freyja`, except for the `git clone` and `cd` commands.

Navigate into the code. You will see two variables like this:

```
model_path = ""  # Update this to your model path
freyja_path = "" # Update this to your freyja path
```
Change them to actual paths. For Freyja, the path will be in `target/release/freyja_rs` (this is a relative path from the Freyja folder).

## Configure Freyja

* Change into the Freyja directory
* Change into the `models` directory
* Move the `config.toml` to the directory indicated in `README.txt`

You will also need to edit the config to indicate the path to the Whisper C++ model you downloaded. Generally, the Ollama host/port will be good, so if you don't know what you're doing, then probably do not touch it.

As for actions, you can take a look at the `example.toml` and all the default ones that I made.

Good luck!
