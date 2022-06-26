use anyhow::{anyhow, Result};
use image2ascii::string2ascii;

pub fn text_to_ascii<S>(text: S) -> Result<String>
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

macro_rules! text_to_ascii_art {
    ($text_value:ident) => {{
        $crate::ascii::text_to_ascii_art($text_value).unwrap()
    }};
}

pub(crate) use text_to_ascii_art;

pub mod static_art {
    pub const HEADER_TITLE: &str = r"
.----------------.  .----------------.  .----------------.  .----------------.
| .--------------. || .--------------. || .--------------. || .--------------. |
| |   ______     | || |      __      | || |      __      | || |  _________   | |
| |  |_   __ \   | || |     /  \     | || |     /  \     | || | |  _   _  |  | |
| |    | |__) |  | || |    / /\ \    | || |    / /\ \    | || | |_/ | | \_|  | |
| |    |  ___/   | || |   / ____ \   | || |   / ____ \   | || |     | |      | |
| |   _| |_      | || | _/ /    \ \_ | || | _/ /    \ \_ | || |    _| |_     | |
| |  |_____|     | || ||____|  |____|| || ||____|  |____|| || |   |_____|    | |
| |              | || |              | || |              | || |              | |
| '--------------' || '--------------' || '--------------' || '--------------' |
 '----------------'  '----------------'  '----------------'  '----------------'
";
}
