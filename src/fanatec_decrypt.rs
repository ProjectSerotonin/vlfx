use std::{ffi::CString, io::Read};
use win32_error::Win32Error;
use winapi::um::wincrypt as WinCrypt;
use thiserror::{Error};

// Fanatec encrypts files in chunks of this
const CHUNK_SIZE: usize = 100000;
const DECRYPTION_KEY: &[u8] = "!!endorfanatec!!".as_bytes();
pub struct FanatecDecrypter {
    windows_key_handle: WinCrypt::HCRYPTKEY
}

impl Drop for FanatecDecrypter {
    fn drop(&mut self) {
        unsafe {
            WinCrypt::CryptDestroyKey(self.windows_key_handle);
        }
    }
}

#[derive(Debug, Error)]
pub enum FanatecDecrypterError {
    #[error("Win32 Error: {0}")]
    WinError(String)
}

fn get_win_error(prefix: &str) -> String {
    format!("{}: {}", prefix, Win32Error::new().to_string())
}

impl FanatecDecrypter {
    /// Creates a ```FanatecDecrypter``` object and derives the key using Window's cryptographic provider
    ///
    /// # Arguments
    ///
    /// * `file` - The buffer to read from
    ///
    pub fn new() -> Result<FanatecDecrypter, FanatecDecrypterError> {
        let provider_name = CString::new("Microsoft Enhanced RSA and AES Cryptographic Provider").expect("CString::new failed");
        let container_name = CString::new("").expect("CString::new failed");

        let provider = unsafe {
            let mut provider: WinCrypt::HCRYPTPROV = std::mem::zeroed();
            let err = WinCrypt::CryptAcquireContextA(&mut provider, container_name.as_ptr(), provider_name.as_ptr(), WinCrypt::PROV_RSA_AES, 0);
            if err == 0 {
                return Err(FanatecDecrypterError::WinError(get_win_error("Error acquiring context")))
            }
            Ok(provider)
        }?;

        let hash: WinCrypt::HCRYPTHASH = unsafe {
            let mut base_data: WinCrypt::HCRYPTHASH = 0;
            let mut err = WinCrypt::CryptCreateHash(provider, WinCrypt::CALG_MD5, 0, 0, &mut base_data);
            if err == 0 {
                return Err(FanatecDecrypterError::WinError(get_win_error("Error creating hash")))
            }
            err = WinCrypt::CryptHashData(base_data, DECRYPTION_KEY.as_ptr(), DECRYPTION_KEY.len() as u32, 0);
            if err == 0 {
                return Err(FanatecDecrypterError::WinError(get_win_error("Error adding data to hash")))
            }
            base_data
        };

        let hkey = unsafe {
            let mut h_key: WinCrypt::HCRYPTKEY = 0;
            let h_key_ptr: *mut usize = &mut h_key;
            let err = WinCrypt::CryptDeriveKey(provider, WinCrypt::CALG_AES_128, hash, 0x800000, h_key_ptr);
            if err == 0 {
                return Err(FanatecDecrypterError::WinError(get_win_error("Error deriving key")))
            }
            h_key
        };

        unsafe {
            WinCrypt::CryptDestroyHash(hash);
        };

        Ok(FanatecDecrypter {
            windows_key_handle: hkey
        })
    }
    /// Decrypts the fanatec firmware file
    ///
    /// # Arguments
    ///
    /// * `file` - The buffer to read from
    ///
    pub fn decrypt<T>(self, file: &mut T) -> Result<Vec<u8>, FanatecDecrypterError>
        where T: Read
    {
        let mut output = Vec::new();

        let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
        let mut is_final = 0;
        while let Ok(read_size) = file.read(&mut buffer[..]) {
            if read_size != CHUNK_SIZE {
                is_final = 1;
            }
            if read_size == 0 {
                break;
            }
            unsafe {
                let mut r_siz = read_size as u32;
                let r_siz_ptr = &mut r_siz as *mut u32;
                let err = WinCrypt::CryptDecrypt(self.windows_key_handle, 0, is_final, 0, buffer.as_mut_ptr(), r_siz_ptr);
                if err == 0 {
                    return Err(FanatecDecrypterError::WinError(get_win_error("Error decrypting file")))
                }
            }
            output.extend_from_slice(&buffer);
        };

        Ok(output)
    }
}
