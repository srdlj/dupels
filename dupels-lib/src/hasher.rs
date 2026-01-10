use std::io::Read;
use std::fmt;
use md5;
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha256Digest};

/// Hash computation strategies
pub trait Hasher: Send + Sync {
    /// Compute hash for the given reader
    fn hash_reader(&self, reader: &mut dyn Read) -> Result<Box<[u8]>, std::io::Error>;
    
    /// Get a human-readable name for this hasher
    fn name(&self) -> &'static str;
    
    /// Clone the hasher for use in other threads
    fn clone_hasher(&self) -> Box<dyn Hasher>;
}

/// Available hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(clap::ValueEnum)]
pub enum HashAlgorithm {
    /// MD5 hash algorithm (fast, 128-bit)
    Md5,
    /// SHA-1 hash algorithm (160-bit)
    Sha1,
    /// SHA-256 hash algorithm (secure, 256-bit)
    Sha256,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Md5
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Md5 => write!(f, "MD5"),
            HashAlgorithm::Sha1 => write!(f, "SHA-1"),
            HashAlgorithm::Sha256 => write!(f, "SHA-256"),
        }
    }
}

impl HashAlgorithm {
    /// Create a hasher instance for this algorithm
    pub fn create_hasher(&self) -> Box<dyn Hasher> {
        match self {
            HashAlgorithm::Md5 => Box::new(Md5Hasher),
            HashAlgorithm::Sha1 => Box::new(Sha1Hasher),
            HashAlgorithm::Sha256 => Box::new(Sha256Hasher),
        }
    }
}

/// MD5 hasher implementation
#[derive(Clone)]
pub struct Md5Hasher;

impl Hasher for Md5Hasher {
    fn hash_reader(&self, reader: &mut dyn Read) -> Result<Box<[u8]>, std::io::Error> {
        let mut context = md5::Context::new();
        let mut buffer = [0u8; crate::CHECKSUM_READ_BUFFER_SIZE];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            context.consume(&buffer[..bytes_read]);
        }
        
        let digest = context.compute();
        Ok(digest.0.to_vec().into_boxed_slice())
    }
    
    fn name(&self) -> &'static str {
        "MD5"
    }
    
    fn clone_hasher(&self) -> Box<dyn Hasher> {
        Box::new(self.clone())
    }
}

/// SHA-1 hasher implementation
#[derive(Clone)]
pub struct Sha1Hasher;

impl Hasher for Sha1Hasher {
    fn hash_reader(&self, reader: &mut dyn Read) -> Result<Box<[u8]>, std::io::Error> {
        let mut hasher = Sha1::new();
        let mut buffer = [0u8; crate::CHECKSUM_READ_BUFFER_SIZE];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            Sha1Digest::update(&mut hasher, &buffer[..bytes_read]);
        }

        Ok(Sha1Digest::finalize(hasher).to_vec().into_boxed_slice())
    }
    
    fn name(&self) -> &'static str {
        "SHA-1"
    }
    
    fn clone_hasher(&self) -> Box<dyn Hasher> {
        Box::new(self.clone())
    }
}

/// SHA-256 hasher implementation
#[derive(Clone)]
pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    fn hash_reader(&self, reader: &mut dyn Read) -> Result<Box<[u8]>, std::io::Error> {
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; crate::CHECKSUM_READ_BUFFER_SIZE];
        
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            Sha256Digest::update(&mut hasher, &buffer[..bytes_read]);
        }
        
        Ok(Sha256Digest::finalize(hasher).to_vec().into_boxed_slice())
    }
    
    fn name(&self) -> &'static str {
        "SHA-256"
    }
    
    fn clone_hasher(&self) -> Box<dyn Hasher> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn hex_string(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    #[test]
    fn test_md5_hasher_empty_input() {
        let hasher = Md5Hasher;
        let mut cursor = Cursor::new(b"");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        let hex = hex_string(&result);
        // MD5 of empty string
        assert_eq!(hex, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_md5_hasher_hello_world() {
        let hasher = Md5Hasher;
        let mut cursor = Cursor::new(b"Hello World");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        let hex = hex_string(&result);
        // MD5 of "Hello World"
        assert_eq!(hex, "b10a8db164e0754105b7a99be72e3fe5");
    }

    #[test]
    fn test_md5_hasher_name() {
        let hasher = Md5Hasher;
        assert_eq!(hasher.name(), "MD5");
    }

    #[test]
    fn test_md5_hasher_clone() {
        let hasher = Md5Hasher;
        let cloned = hasher.clone_hasher();
        assert_eq!(cloned.name(), "MD5");
    }

    #[test]
    fn test_sha1_hasher_empty_input() {
        let hasher = Sha1Hasher;
        let mut cursor = Cursor::new(b"");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        let hex = hex_string(&result);
        // SHA-1 of empty string
        assert_eq!(hex, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn test_sha1_hasher_hello_world() {
        let hasher = Sha1Hasher;
        let mut cursor = Cursor::new(b"Hello World");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        let hex = hex_string(&result);
        // SHA-1 of "Hello World"
        assert_eq!(hex, "0a4d55a8d778e5022fab701977c5d840bbc486d0");
    }

    #[test]
    fn test_sha1_hasher_name() {
        let hasher = Sha1Hasher;
        assert_eq!(hasher.name(), "SHA-1");
    }

    #[test]
    fn test_sha1_hasher_clone() {
        let hasher = Sha1Hasher;
        let cloned = hasher.clone_hasher();
        assert_eq!(cloned.name(), "SHA-1");
    }

    #[test]
    fn test_sha256_hasher_empty_input() {
        let hasher = Sha256Hasher;
        let mut cursor = Cursor::new(b"");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        let hex = hex_string(&result);
        // SHA-256 of empty string
        assert_eq!(hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_sha256_hasher_hello_world() {
        let hasher = Sha256Hasher;
        let mut cursor = Cursor::new(b"Hello World");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        let hex = hex_string(&result);
        // SHA-256 of "Hello World"
        assert_eq!(hex, "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e");
    }

    #[test]
    fn test_sha256_hasher_name() {
        let hasher = Sha256Hasher;
        assert_eq!(hasher.name(), "SHA-256");
    }

    #[test]
    fn test_sha256_hasher_clone() {
        let hasher = Sha256Hasher;
        let cloned = hasher.clone_hasher();
        assert_eq!(cloned.name(), "SHA-256");
    }

    #[test]
    fn test_hash_algorithm_create_hasher() {
        let md5_hasher = HashAlgorithm::Md5.create_hasher();
        let sha1_hasher = HashAlgorithm::Sha1.create_hasher();
        let sha256_hasher = HashAlgorithm::Sha256.create_hasher();

        assert_eq!(md5_hasher.name(), "MD5");
        assert_eq!(sha1_hasher.name(), "SHA-1");
        assert_eq!(sha256_hasher.name(), "SHA-256");
    }

    #[test]
    fn test_hash_algorithm_default() {
        let default_algo = HashAlgorithm::default();
        assert_eq!(default_algo, HashAlgorithm::Md5);
    }

    #[test]
    fn test_hash_algorithm_display() {
        assert_eq!(format!("{}", HashAlgorithm::Md5), "MD5");
        assert_eq!(format!("{}", HashAlgorithm::Sha1), "SHA-1");
        assert_eq!(format!("{}", HashAlgorithm::Sha256), "SHA-256");
    }

    #[test]
    fn test_hasher_with_large_input() {
        let large_data = vec![b'A'; 10000]; // 10KB of 'A' characters
        
        let md5_hasher = Md5Hasher;
        let sha1_hasher = Sha1Hasher;
        let sha256_hasher = Sha256Hasher;

        let mut cursor1 = Cursor::new(&large_data);
        let mut cursor2 = Cursor::new(&large_data);
        let mut cursor3 = Cursor::new(&large_data);

        let md5_result = md5_hasher.hash_reader(&mut cursor1);
        let sha1_result = sha1_hasher.hash_reader(&mut cursor2);
        let sha256_result = sha256_hasher.hash_reader(&mut cursor3);

        assert!(md5_result.is_ok());
        assert!(sha1_result.is_ok());
        assert!(sha256_result.is_ok());

        // Verify different algorithms produce different hashes
        let md5_hex = hex_string(&md5_result.unwrap());
        let sha1_hex = hex_string(&sha1_result.unwrap());
        let sha256_hex = hex_string(&sha256_result.unwrap());

        assert_ne!(md5_hex, sha1_hex);
        assert_ne!(md5_hex, sha256_hex);
        assert_ne!(sha1_hex, sha256_hex);
    }

    #[test]
    fn test_hasher_consistency() {
        let test_data = b"The quick brown fox jumps over the lazy dog";
        
        // Test that same input produces same output
        let md5_hasher = Md5Hasher;
        
        let mut cursor1 = Cursor::new(test_data);
        let mut cursor2 = Cursor::new(test_data);
        
        let result1 = md5_hasher.hash_reader(&mut cursor1).unwrap();
        let result2 = md5_hasher.hash_reader(&mut cursor2).unwrap();
        
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_hasher_trait_object() {
        let test_data = b"test data";
        
        let hashers: Vec<Box<dyn Hasher>> = vec![
            Box::new(Md5Hasher),
            Box::new(Sha1Hasher),
            Box::new(Sha256Hasher),
        ];

        for hasher in hashers {
            let mut cursor = Cursor::new(test_data);
            let result = hasher.hash_reader(&mut cursor);
            assert!(result.is_ok());
            assert!(!result.unwrap().is_empty());
        }
    }
}
