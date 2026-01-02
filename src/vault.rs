use std::fs::{self, File};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use rand::RngCore;
use rand::rngs::OsRng;
use crate::crypto::{CryptoContext, derive_key};
use argon2::password_hash::SaltString;
use aes_gcm::aead::OsRng as AeadOsRng;

const VAULT_FILE: &str = "VAULT.DAT";
const UNLOCKED_DIR: &str = "UNLOCKED_CONTENTS";

pub fn lock(password: &str) -> std::io::Result<()> {
    
    if !Path::new(UNLOCKED_DIR).exists() {
        return Ok(()); // Nothing to lock
    }

    let mut buffer = Vec::new();
    // Create zip
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buffer));
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        
        // Walk dir
        for entry in walkdir::WalkDir::new(UNLOCKED_DIR) {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let name = path.strip_prefix(UNLOCKED_DIR).unwrap().to_str().unwrap();
                zip.start_file(name, options)?;
                let mut f = File::open(path)?;
                let mut contents = Vec::new();
                f.read_to_end(&mut contents)?;
                zip.write_all(&contents)?;
            }
        }
        zip.finish()?;
    }

    // 2. Encrypt buffer
    // Generate Salt
    let salt = SaltString::generate(&mut OsRng); // This logic needs import adjustment if fails
    // Hack: Just use random bytes for salt
    let mut salt_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut salt_bytes);
    
    // Argon2 is bit complex to setup manually so we use our crypto helper
    // wait, we need to pass the salt to `derive_key`.
    let key = derive_key(password, &salt_bytes);
    let ctx = CryptoContext::new(&key);
    
    let (ciphertext, nonce) = ctx.encrypt(&buffer);

    // 3. Write VAULT.DAT format: [Salt 16][Nonce 12][Ciphertext]
    let mut f = File::create(VAULT_FILE)?;
    f.write_all(&salt_bytes)?;
    f.write_all(&nonce)?;
    f.write_all(&ciphertext)?;

    // 4. Shred UNLOCKED_DIR
    std::fs::remove_dir_all(UNLOCKED_DIR)?;
    
    Ok(())
}

pub fn unlock(password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if !Path::new(VAULT_FILE).exists() {
        // No vault, maybe create empty directory?
        fs::create_dir_all(UNLOCKED_DIR)?;
        return Ok(true); // Treated as new
    }

    let mut f = File::open(VAULT_FILE)?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)?;

    if content.len() < 16 + 12 {
        return Err("Vault corrupted".into());
    }

    let salt = &content[0..16];
    let nonce: &[u8; 12] = &content[16..28].try_into()?;
    let ciphertext = &content[28..];

    let key = derive_key(password, salt);
    let ctx = CryptoContext::new(&key);

    match ctx.decrypt(ciphertext, nonce) {
        Ok(plaintext) => {
            // Unzip
            let reader = std::io::Cursor::new(plaintext);
            let mut zip = zip::ZipArchive::new(reader)?;
            
            fs::create_dir_all(UNLOCKED_DIR)?;
            
            for i in 0..zip.len() {
                let mut file = zip.by_index(i)?;
                let outpath = Path::new(UNLOCKED_DIR).join(file.name());
                
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
            Ok(true)
        },
        Err(_) => Ok(false), // Header ok, auth failed
    }
}

pub fn shred_vault() -> std::io::Result<()> {
    if Path::new(VAULT_FILE).exists() {
        let meta = fs::metadata(VAULT_FILE)?;
        let len = meta.len();
        let mut f = File::options().write(true).open(VAULT_FILE)?;
        
        // Multipass overwrite
        let zeros = vec![0u8; 4096];
        let passes = 3;
        
        for _ in 0..passes {
            f.seek(SeekFrom::Start(0))?;
            let mut written = 0;
            while written < len {
                let to_write = std::cmp::min(zeros.len() as u64, len - written);
                f.write_all(&zeros[0..to_write as usize])?;
                written += to_write;
            }
            f.flush()?;
        }
        
        drop(f);
        fs::remove_file(VAULT_FILE)?;
    }
    // Also remove directory if exists to prevent confusion
    if Path::new(UNLOCKED_DIR).exists() {
         fs::remove_dir_all(UNLOCKED_DIR)?;
    }
    Ok(())
}
