use std::net::{SocketAddr, UdpSocket};

use anyhow::{Error, Ok, Result};

use crate::osc::message::OscMessage;

/// # VRC OSC Client
pub struct VRCOSCClient {
    bind: Option<SocketAddr>,
    target: Option<SocketAddr>,
    socket: Option<UdpSocket>,
}

impl VRCOSCClient {
    /// - use "0.0.0.0:0" as default bind address
    /// - use "127.0.0.1:9000" as default connect address
    pub fn new() -> Result<Self> {
        Ok(Self {
            bind: Some("0.0.0.0:0".parse().map_err(Error::new)?),
            target: Some("127.0.0.1:9000".parse().map_err(Error::new)?),
            socket: None,
        })
    }

    pub fn replace_bind(&mut self, addr: SocketAddr) -> Option<SocketAddr> {
        self.bind.replace(addr)
    }

    pub fn replace_target(&mut self, addr: SocketAddr) -> Option<SocketAddr> {
        self.target.replace(addr)
    }

    pub fn start(&mut self) -> Result<()> {
        let Some(target_addr) = self.target.as_ref() else {
            return Err(Error::msg("target is empty"));
        };

        let Some(bind_addr) = self.bind.as_ref() else {
            return Err(Error::msg("no bind address"));
        };

        let socket = UdpSocket::bind(bind_addr).map_err(Error::new)?;
        let _ = socket.connect(target_addr).map_err(Error::new)?;
        self.socket = Some(socket);
        Ok(())
    }

    /// must be called after start()
    pub fn send(&self, msg: impl Into<OscMessage>) -> Result<usize> {
        if let Some(s) = self.socket.as_ref() {
            s.send(msg.into().encode().as_slice()).map_err(Error::new)
        } else {
            Err(Error::msg("no active socket"))
        }
    }
}

#[cfg(test)]
mod test {
    use std::{io, thread::sleep, time::Duration};

    use anyhow::{Error, Ok, Result};

    use crate::{
        client::VRCOSCClient,
        vrchat::{
            avatar::{AvatarEyeheight, AvatarEyeheightMax, AvatarEyeheightScalingAllowed},
            chatbox::{ChatboxInput, ChatboxTyping},
            input::{
                InputJump, InputMoveBackward, InputMoveForward, InputMoveLeft, InputMoveRight,
            },
        },
    };

    const SHORT_PRESS_PERIOD: u64 = 60;

    #[test]
    /// require console input
    fn test_chatbox_input() -> Result<()> {
        let mut client = VRCOSCClient::new()?;
        client.start()?;
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).map_err(Error::new)?;
        input = format!(
            "content={:?}",
            input.strip_suffix("\r\n").expect("what's wrong?")
        );
        let _ = client.send(ChatboxInput::new(&input))?;
        Ok(())
    }

    #[test]
    /// require console input
    fn test_chatbox_typing() -> Result<()> {
        let mut client = VRCOSCClient::new()?;
        client.start()?;
        let _ = client.send(ChatboxTyping::new(true))?;
        Ok(())
    }

    #[test]
    /// require console input
    fn test_avatar_height_change() -> Result<()> {
        let mut client = VRCOSCClient::new()?;
        client.start()?;
        let _ = client.send(AvatarEyeheight::new(10.0))?;
        Ok(())
    }

    #[test]
    /// require console input
    fn test_avatar_height_max_min_change() -> Result<()> {
        let mut client = VRCOSCClient::new()?;
        client.start()?;
        let _ = client.send(AvatarEyeheightScalingAllowed::new(true))?;
        let _ = client.send(AvatarEyeheightMax::new(20.0))?;
        let _ = client.send(AvatarEyeheightMax::new(0.01))?;
        Ok(())
    }

    #[test]
    fn test_input_jump() -> Result<()> {
        let mut client = VRCOSCClient::new()?;
        client.start()?;

        for i in 0..99 {
            println!("jump-{i:?}");
            let _ = client.send(InputJump::new(true))?;
            sleep(Duration::from_millis(SHORT_PRESS_PERIOD));
            let _ = client.send(InputJump::new(false))?;
            sleep(Duration::from_millis(580));
        }
        Ok(())
    }

    #[test]
    fn test_move_around_and_jump() -> Result<()> {
        let mut client = VRCOSCClient::new()?;
        client.start()?;
        for _ in 0..999 {
            let _ = turn_a_smooth_round(&client)?;
            let _ = jump_until_down(&client)?;
        }

        Ok(())
    }

    fn jump_until_down(client: &VRCOSCClient) -> Result<()> {
        let _ = client.send(InputJump::new(true))?;
        sleep(Duration::from_millis(SHORT_PRESS_PERIOD));
        let _ = client.send(InputJump::new(false))?;
        sleep(Duration::from_millis(600));
        Ok(())
    }

    fn turn_a_smooth_round(client: &VRCOSCClient) -> Result<()> {
        let press_ms = 260;
        let _ = client.send(InputMoveForward::new(true))?;
        sleep(Duration::from_millis(press_ms / 2));
        let _ = client.send(InputMoveLeft::new(true))?;
        sleep(Duration::from_millis(press_ms / 2));
        let _ = client.send(InputMoveForward::new(false))?;
        let _ = client.send(InputMoveBackward::new(true))?;
        sleep(Duration::from_millis(press_ms / 2));
        let _ = client.send(InputMoveLeft::new(false))?;
        let _ = client.send(InputMoveRight::new(true))?;
        sleep(Duration::from_millis(press_ms / 2));
        let _ = client.send(InputMoveBackward::new(false))?;
        sleep(Duration::from_millis(press_ms / 2));
        let _ = client.send(InputMoveRight::new(false))?;
        sleep(Duration::from_millis(SHORT_PRESS_PERIOD));
        Ok(())
    }
}
