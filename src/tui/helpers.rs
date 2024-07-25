pub fn valid_length(text: &str, min: usize, msg: &str) -> Result<(), String> {
    if text.len() > min {
        Ok(())
    } else {
        Err(msg.to_owned())
    }
}
