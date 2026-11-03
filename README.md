# vrc-osc-api

👇 AI generated 👇

A small Rust SDK for sending OSC messages to VRChat over UDP.

This crate is intended for personal experimentation and lightweight integration work. It focuses on the OSC messages most commonly used from VRChat tooling and keeps the API small and direct.

## Features

- UDP client with configurable bind and target addresses
- OSC message builder and encoder
- VRChat chatbox helpers:
  - `/chatbox/input`
  - `/chatbox/typing`
- VRChat avatar scaling helpers:
  - `/avatar/eyeheight`
  - `/avatar/eyeheightmin`
  - `/avatar/eyeheightmax`
  - `/avatar/eyeheightscalingallowed`

## Requirements

- Rust 2024 edition
- VRChat with OSC enabled

VRChat usually listens for OSC input on `127.0.0.1:9000`. If your setup uses a different target address, configure it before starting the client.

## Installation

Add the crate to your project:

```toml
[dependencies]
vrc-osc-api = "0.0.2"
```

## Quick start

Send a chatbox message:

```rust
use std::error::Error;
use vrc_osc_api::{client::VRCOSCClient, vrchat::chatbox::ChatboxInput};

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = VRCOSCClient::new()?;
    client.start()?;
    client.send(ChatboxInput::new("Hello from Rust"))?;
    Ok(())
}
```

Customize the bind/target addresses before starting:

```rust
let mut client = VRCOSCClient::new()?;
client.replace_bind("0.0.0.0:0".parse()?);
client.replace_target("127.0.0.1:9000".parse()?);
client.start()?;
```

Notes:

- `start()` must be called before `send()`.
- `send()` accepts anything that can be converted into an `OscMessage`.
- `ChatboxInput` and the built-in VRChat message wrappers are supported directly.

## Building OSC messages manually

The low-level message types are also available when you want to build a packet yourself:

```rust
use vrc_osc_api::osc::{message::OscMessage, value::OSCValue};

let message = OscMessage::new("/example")
    .arg(OSCValue::Int32(42))
    .arg(OSCValue::Float32(0.5));

let packet: Vec<u8> = message.encode();
```

## Current status and caveats

This project is still under active development and should be treated as experimental rather than a fully-complete OSC library.

Some important limits to be aware of:

- not every OSC value variant is implemented for encoding
- blob, timetag, and some alternate OSC types are not fully supported yet
- string padding and some low-level encoding details are still being refined
- generated packets should be validated against your VRChat setup before depending on them in production automation

The current API is intentionally lightweight and is best used for direct, tested VRChat message sending rather than as a general-purpose OSC framework.

## References

- [OSC Specification 1.0](https://opensoundcontrol.stanford.edu/spec-1_0.html)
- [VRChat OSC Overview](https://docs.vrchat.com/docs/osc-overview)
