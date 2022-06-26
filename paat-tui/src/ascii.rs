use anyhow::{anyhow, Result};
use image2ascii::string2ascii;

pub fn text_to_ascii_art<S>(text: S) -> Result<String>
where
    S: AsRef<str>,
{
    if let Ok(array) = string2ascii(text.as_ref(), 15f32, 'P', None, None) {
        let mut ascii_string = array.to_lines().join("\n");
        ascii_string.push('\n');
        return Ok(ascii_string);
    }
    Err(anyhow!("Failed to convert string to ascii art"))
}
