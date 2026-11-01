pub fn realign_bytes(buffer: &mut Vec<u8>) {
    let align = 4 - (buffer.len() & 3);
    for _ in 0..align {
        buffer.push(0);
    }
}
