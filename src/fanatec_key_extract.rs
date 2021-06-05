use std::{error::Error, fmt, io::{Read, Seek, SeekFrom}};
use sha1::{Sha1, Digest};
use std::io;
const KEY_SIZE: usize = 16;

fn hash_to_offset(hash: &str) -> Option<usize> {
    match hash {
        "64ee2e765dca4f27e67ed4dfa320192543fa69ed" => Some(0x1585F0),
        _ => None
    }
}
#[derive(Debug)]
pub enum FanatecKeyExtractError {
    BinaryNotInDatabase,
    IOError(std::io::Error)
}

impl From<std::io::Error> for FanatecKeyExtractError {
    fn from(err: std::io::Error) -> Self {
        FanatecKeyExtractError::IOError(err)
    }
}

impl Error for FanatecKeyExtractError {}

impl fmt::Display for FanatecKeyExtractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FanatecKeyExtractError::BinaryNotInDatabase => write!(f, "Binary not found in offset DB"),
            FanatecKeyExtractError::IOError(e) => write!(f, "IO Error: {}", &e.to_string())
        }
    }
}

pub fn extract_key(mut file: impl Read + Seek) -> Result<[u8; KEY_SIZE], FanatecKeyExtractError> {
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    let offset = hash_to_offset(&format!("{:x}", hash)).ok_or(FanatecKeyExtractError::BinaryNotInDatabase)?;
    let mut key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    file.seek(SeekFrom::Start(offset as u64))?;
    file.read(&mut key)?;
    Ok(key)
}