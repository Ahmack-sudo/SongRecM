# SongRec

SongRec is an open-source Shazam client for Linux, written in Rust.

![Screenshot](Screenshot.png?raw=true)

Features:

* Recognize audio from an arbitrary audio file.
* Recognize audio from the microphone.
* Usage from both GUI and command line (for the file recognition part).
* Provide an history of the recognized songs on the GUI, exportable to CSV.
* Continuous song detection from the microphone, with the ability to choose your input device.
* Generate a lure from a song that, when played, will fool Shazam into thinking that it is the concerned song.

A (command-line only) Python version, which I made before rewriting in Rust for performance, is also available for demonstration purposes. It supports file recognition only.

## How it works

For useful information about how audio fingerprinting works, you may want to read [this article](http://coding-geek.com/how-shazam-works/). To be put simply, Shazam generates a spectrogram (a time/frequency 2D graph of the sound, with amplitude at intersections) of the sound, and maps out the frequency peaks from it (which should match key points of the harmonics of voice or of certains instruments).

Shazam also downsamples the sound at 16 KHz before processing, and cuts the sound in four bands of 250-520 Hz, 520-1450 Hz, 1450-3500 Hz, 3500-5500 Hz (so that if a band is too much scrambled by noise, recognition from other bands may apply). The frequency peaks are then sent to the servers, which subsequently look up the strongest peaks in a database, in order look for the simultaneous presence of neighboring peaks both in the associated reference fingerprints and in the fingerprint we sent.

Hence, the Shazam fingerprinting algorithm, as implemented by the client, is fairly simple, as much of the processing is done server-side. The general functionment of Shazam has been documented in public [research papers](https://www.ee.columbia.edu/~dpwe/papers/Wang03-shazam.pdf) and patents.

## Installation

Here are a few ways to install and run the application:

Using AUR (Archlinux):

```bash
yay -S songrec
songrec
```

Using apt with PPA (Ubuntu, supported 18.04, 20.04, 20.10):

```bash
sudo apt-add-repository ppa:marin-m/songrec -y -u
sudo apt install songrec -y
songrec
```

Using Flatpak (all distributions) (NOTE: with Flatpak, the GUI should work fine but some of the CLI features may not be usable due to filesystem sandboxing restrictions):

```bash
sudo apt install flatpak -y
flatpak remote-add flathub https://flathub.org/repo/flathub.flatpakrepo --if-not-exists
flatpak install --user flathub com.github.marinm.songrec -y
flatpak run com.github.marinm.songrec
```

Using Cargo (all distributions, dependencies given for Ubuntu/Debian, if your `rustc` version is not recent enough please refer to the instructions below):

```bash
sudo apt install cargo rustc -y
echo 'export PATH="$HOME/.cargo/bin:$PATH"' | tee -a ~/.profile ~/.bashrc
source ~/.bashrc

sudo apt install build-essential libasound2-dev libgtk-3-dev libssl-dev -y
cargo install songrec
songrec
```

## Compilation

(**WARNING**: Remind to compile the code in "--release" mode for correct performance.)

### Installing Rust

First, you need to [install the Rust compiler and package manager](https://www.rust-lang.org/tools/install). It has been observed to work with `rustc` 1.43.0 to the current rust 1.47.0.

Install Rust and put it in path, for all distributions:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Type "1"
# Login and reconnect to add Rust to the $PATH, or run:
source $HOME/.cargo/env

# If you already installed Rust, then update it:
rustup update
```

Install dependent libraries (nothing exotic):

```bash
sudo apt install build-essential libasound2-dev libgtk-3-dev libssl-dev
```

### Compiling the project

This will compile and run the projet:

```bash
# For the stable release:
cargo install songrec
songrec

# For the Github tree:
git clone git@github.com:marin-m/songrec.git
cd songrec
cargo run --release
```

For the latter, you will then find the project's binary (that you will be able to move or execute directly) at `target/release/songrec`.

## Sample usage

Passing no arguments or using the `gui` subcommand will launch the GUI, and try to recognize audio real-time as soon as the application is launched:

```
./songrec
./songrec gui
```

Using the `gui-norecording` subcommand will launch the GUI without recognizing audio as soon as the software is started (you will need to click the "Turn on microphone recognition" button to do so):

```
./songrec gui-norecording
```

The following subcommand will try to recognize audio from the middle of an audio file, and print the JSON response from Shazam servers (MP3, FLAC, WAV, OGG formats should be accepted):

```
./songrec audio-file-to-recognized-song sound_file.mp3
```

The following subcommands will do the same with an intermediary step, manipulating data-URI audio fingerprints as used by Shazam internally:

```
./songrec audio-file-to-fingerprint sound_file.mp3
./songrec fingerprint-to-recognized-song 'data:audio/vnd.shazam.sig;base64,...'
```

The following will produce back hearable tones from a given fingerprint, that should be able to fool Shazam into thinking that this is the original song (either to the default audio output device, or to a .WAV file):

```
./songrec fingerprint-to-lure 'data:audio/vnd.shazam.sig;base64,...'
./songrec fingerprint-to-lure 'data:audio/vnd.shazam.sig;base64,...' /tmp/output.wav
```

When using the application, you may notice that certain information will be saved to `~/.local/share/SongRec` (or an equivalent directory depending on your operating system), including the CSV-format list of the last recognized songs and the last selected microphone input device (so that it is chosen back when restarting the app). You may want to delete this directory in case of persistent issues.

## Legal

This software is released under the [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.html) license. It was created with the intent of providing interoperability between the remote Shazam services and Linux-based deskop systems.

Please note that in certain countries located outside of the European Union, especially the United States, software patents may apply.

