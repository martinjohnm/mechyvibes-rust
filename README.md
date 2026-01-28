

# Mechyvibes

A Mechvibes alternative written in Rust

## Overview

RustyVies is a lightweight, cross-platform mechanical keyboard sound simulator, inspired by Mechvibes and written entirely in Rust.

The application plays realistic mechanical keyboard sounds (key press / release) in real time while typing, with a strong focus on low latency, safe concurrency, and efficient audio event handling using Rust’s message-passing (tx / rx) concurrency model.

## Features

- ⌨️ Real-time mechanical keyboard sound playback

- 🔊 Support for key press / release sound events

- 🧵 Concurrent event handling using Rust channels (tx / rx)

- ⚡ Low-latency audio playback

- 🛡️ Memory-safe and data-race-free by design

- 🦀 Built entirely in Rust

## Architecture

RustyVies is built around an event-driven, concurrent pipeline:

![Descriptive alt text](architecture.png)

## Why channels?

- Keyboard events are produced rapidly and asynchronously

- Audio playback must be non-blocking

- Channels allow:

  - decoupling input capture from audio playback

  - clean separation of responsibilities

  - safe concurrency without shared mutable state

## Concurrency Model

- Keyboard input runs in its own execution context

- Each key event is sent through a channel (Sender)

- The audio engine listens via a Receiver

- Sound playback happens in a dedicated worker thread

This ensures:

- no blocking on key events

- consistent audio timing

- zero data races

## Tech Stack

- Language: Rust 🦀

- Concurrency:

  - std::thread

  - flume::{Receiver, Sender}
  
  - rodio_wav_fix

- Audio: Rust audio playback library (e.g. rodio / cpal)

- Build Tool: Cargo

## Project structure

mechyvibes-rust/
├── src/
|   ├── args.rs            # Args parser using clap
│   ├── keyboard.rs        # Keyboard event listener
│   ├── main.rs            # App entry point
│   ├── sound.rs           # Audio playback engine
│   └── start.rs          # Sound profiles & settings
├── Soundpack/             # Key sound files
├── Cargo.toml
└── README.md


## How It Works

- Keyboard events are captured globally

- Each key press / release is converted into an event

- Events are sent via tx to the audio thread

- The audio engine plays the corresponding sound

- All components communicate via channels only

## Run 

- give the sound file name as the parameter

    ### cargo run Soundpacks/cherrymx-blue-abs
