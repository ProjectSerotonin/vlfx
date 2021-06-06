use std::{io::{Read, Seek, SeekFrom}};
use sha1::{Sha1, Digest};
use std::io;
use thiserror::Error;
const KEY_SIZE: usize = 16;

// Maps the hash for the FwClubSportBaseUpdater.exe file to the offset where the key is located
fn hash_to_offset(hash: &str) -> Option<usize> {
    match hash {
        "64ee2e765dca4f27e67ed4dfa320192543fa69ed" => Some(0x1585F0),
        _ => None
    }
}
#[derive(Debug, Error)]
pub enum FanatecKeyExtractError {
    #[error("Binary not found in DB")]
    BinaryNotInDatabase,
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error)
}

/// Tries to extract the key from the given FwClubSportBaseUpdater.exe
///
/// # Arguments
///
/// * `file` - The buffer to read from
///
pub fn extract_key<T>(mut file: T) -> Result<[u8; KEY_SIZE], FanatecKeyExtractError>
    where T: Read + Seek
{
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    let offset = hash_to_offset(&format!("{:x}", hash)).ok_or(FanatecKeyExtractError::BinaryNotInDatabase)?;
    let mut key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    file.seek(SeekFrom::Start(offset as u64))?;
    file.read(&mut key)?;
    Ok(key)
}