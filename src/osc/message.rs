use crate::{osc::value::OSCValue, tools::byte_tools::realign_bytes};

const ADDRESS_SEPERATOR: char = '/';
const OSC_TYPE_TAG: u8 = ',' as u8;

pub struct OscMessage {
    address: String,
    args: Vec<OSCValue>,
}

impl OscMessage {
    pub fn new(address: impl Into<String>) -> Self {
        let addr = address.into();
        if !addr.starts_with(ADDRESS_SEPERATOR) {
            todo!()
        }
        Self {
            address: addr,
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, value: impl Into<OSCValue>) -> Self {
        self.args.push(value.into());
        self
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        // initialize osc address
        for ele in self.address.as_bytes() {
            buffer.push(*ele);
        }
        realign_bytes(&mut buffer);
        // initialize osc type tag
        buffer.push(OSC_TYPE_TAG);
        for arg in &self.args {
            buffer.push(arg.get_tag());
        }
        realign_bytes(&mut buffer);
        // write value
        for ele in &self.args {
            ele.write_to_buffer(&mut buffer);
        }

        buffer
    }
}
