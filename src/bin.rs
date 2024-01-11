use polkavm::ProgramBlob;

const CHUNK_SIZE: usize = 16;

//------------------------------------------------------------------------------

pub type LoadError = String;

#[derive(Clone, Debug)]
pub struct Binary {
    // TODO: Use proper types rather than strings.
    pub memory: Vec<String>,
    pub code: Vec<String>,
}

impl Binary {
    pub fn new(memory: Vec<String>, code: Vec<String>) -> Self {
        Self { memory, code }
    }
}

//------------------------------------------------------------------------------

pub fn load(data: &[u8]) -> Result<Binary, LoadError> {
    disassemble(&data).and_then(|code| {
        let memory = parse_data(&data, CHUNK_SIZE);
        Ok(Binary::new(memory, code))
    })
}

fn parse_data(data: &[u8], chunk_size: usize) -> Vec<String> {
    data.chunks(chunk_size)
        .enumerate()
        .map(move |(index, chunk)| {
            let current_offset = index * chunk_size; // Calculate offset here

            // Initialize the strings with a capacity that avoids further allocation.
            // 23 for hex_part: 2 chars per byte and 1 space, except after the last byte.
            // 8 for text_part: 1 char per byte.
            let mut hex_part = String::with_capacity(23);
            let mut text_part = String::with_capacity(8);

            for &byte in chunk {
                // Write the hex representation directly into hex_part.
                use std::fmt::Write;
                write!(hex_part, "{:02x} ", byte).expect("Writing to a String should never fail");
                // Append ASCII representation or '.' to text_part.
                text_part.push(if (32..=126).contains(&byte) { byte as char } else { '.' });
            }

            // Trim the trailing space from the hex_part and pad if necessary.
            let hex_part = hex_part.trim_end().to_string();
            let hex_part_padded = format!("{:23}", hex_part);

            // Pad text_part if necessary.
            let text_part_padded = format!("{:<8}", text_part);

            // Format the output string with the current offset
            let output = format!("{:06x} {} {}", current_offset, hex_part_padded, text_part_padded);
            output
        })
        .collect()
}

fn disassemble(data: &[u8]) -> Result<Vec<String>, LoadError> {
    let blob = ProgramBlob::parse(data);
    if blob.is_err() {
        return Err("Failed to parse blob".into());
    }

    let blob = blob.unwrap();

    let mut result = vec![];
    for (i, maybe_instruction) in blob.instructions().enumerate() {
        match maybe_instruction {
            Ok(instruction) => {
                result.push(instruction.to_string());
            }
            Err(error) => {
                return Err(format!(
                    "ERROR: failed to parse raw instruction from blob. nth: {} Error: {}\n",
                    i, error
                ));
            }
        }
    }

    Ok(result)
}
