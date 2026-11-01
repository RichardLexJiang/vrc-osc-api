# vrc-osc-api 

> AI generated

A small, personal Rust SDK for sending OSC messages to VRChat over UDP. The project is currently a work in progress, intended for self-use and experimentation rather than as a stable, general-purpose OSC implementation.

## What it includes

- A UDP client with configurable bind and target addresses.
- An OSC message builder.
- A VRChat chatbox input message (`/chatbox/input`).

## Requirements

- Rust (edition 2024).
- VRChat running with OSC enabled.

VRChat's default OSC input address is `127.0.0.1:9000`. If your setup uses a different address, configure the client target accordingly.

## Use from a Rust project

If this repository is available locally, add it as a path dependency:

```toml
[dependencies]
vrc-osc-api = "0.0.1"
```

Then send a chatbox message:

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

The client defaults to binding `0.0.0.0:0` and sending to `127.0.0.1:9000`. Set different addresses before starting if needed:

```rust
let mut client = VRCOSCClient::new()?;
client.replace_bind("0.0.0.0:0".parse()?);
client.replace_target("127.0.0.1:9000".parse()?);
client.start()?;
```

`start()` must be called before `send()`. `send()` accepts a value convertible into an `OscMessage`; `ChatboxInput` is supported directly.

## Creating an OSC message

The low-level message builder is available for constructing messages:

```rust
use vrc_osc_api::osc::{message::OscMessage, value::OSCValue};

let message = OscMessage::new("/example")
    .arg(OSCValue::Int32(42))
    .arg(OSCValue::Float32(0.5));

let packet: Vec<u8> = message.encode();
```

## Current limitations

This SDK is under active development and does not yet implement all OSC value encodings. `OSCValue` variants such as blobs, 64-bit values, and several other types currently panic if encoded. Use only the variants whose encoding you have verified, and treat generated packets as experimental.

The chatbox API currently exposes only the message text through `ChatboxInput::new`; keyboard-bypass and notification-sound options use their defaults. The OSC encoder and its string padding are also incomplete, so verify packet behavior against your VRChat setup before relying on it.

