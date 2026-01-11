use std::io::Read;
use std::fmt;
use md5;
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha256Digest};
use hound::{WavReader, SampleFormat};
use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

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
    /// Audio Perceptual Hash algorithm
    AudioPHash,
    /// Image Perceptual Hash algorithm
    ImagePHash
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
            HashAlgorithm::AudioPHash => write!(f, "Audio-pHash"),
            HashAlgorithm::ImagePHash => write!(f, "Image-pHash"),
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
            HashAlgorithm::AudioPHash => Box::new(AudioPHasher),
            //HashAlgorithm::ImagePHash =>
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

/// Audio Perceptual Hash implementation  
#[derive(Clone)]
pub struct AudioPHasher;

impl AudioPHasher {
    /// Load audio data from reader and convert to mono f32 samples
    fn load_audio_data(&self, data: &[u8]) -> Result<Vec<f32>, std::io::Error> {
        let cursor = std::io::Cursor::new(data);
        let mut wav_reader = WavReader::new(cursor).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("WAV parsing error: {}", e))
        })?;
        
        let spec = wav_reader.spec();
        let mut samples = Vec::new();

        // Convert samples to f32 and handle multi-channel by taking first channel
        match spec.sample_format {
            SampleFormat::Float => {
                for (i, sample) in wav_reader.samples::<f32>().enumerate() {
                    let s = sample.map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Sample error: {}", e))
                    })?;
                    // Take only first channel (convert to mono)
                    if i % spec.channels as usize == 0 {
                        samples.push(s);
                    }
                }
            }
            SampleFormat::Int => {
                let max_amplitude = 2_i32.pow(spec.bits_per_sample as u32 - 1) as f32;
                for (i, sample) in wav_reader.samples::<i32>().enumerate() {
                    let s = sample.map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Sample error: {}", e))
                    })?;
                    // Take only first channel (convert to mono)
                    if i % spec.channels as usize == 0 {
                        samples.push(s as f32 / max_amplitude);
                    }
                }
            }
        }

        Ok(samples)
    }

    /// Compute spectral fingerprint from audio samples
    fn compute_spectral_hash(&self, samples: Vec<f32>) -> Result<u64, std::io::Error> {
        let window_size = 1024;
        let hop_size = 512;
        let num_bands = 8;
        
        if samples.len() < window_size {
            return Ok(0); // Return zero hash for very short audio
        }

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(window_size);
        
        let mut band_energies = vec![0.0f32; num_bands];
        let mut window_count = 0;

        // Process overlapping windows
        for start in (0..samples.len().saturating_sub(window_size)).step_by(hop_size) {
            let mut buffer: Vec<Complex<f32>> = samples[start..start + window_size]
                .iter()
                .enumerate()
                .map(|(i, &s)| {
                    // Apply Hanning window
                    let window = 0.5 * (1.0 - (2.0 * PI * i as f32 / (window_size - 1) as f32).cos());
                    Complex::new(s * window, 0.0)
                })
                .collect();

            // Compute FFT
            fft.process(&mut buffer);

            // Compute energy in frequency bands (logarithmically spaced)
            let nyquist = window_size / 2;
            for band in 0..num_bands {
                let start_bin = (nyquist as f32 * (band as f32 / num_bands as f32).powf(2.0)) as usize;
                let end_bin = (nyquist as f32 * ((band + 1) as f32 / num_bands as f32).powf(2.0)) as usize;
                
                let mut energy = 0.0;
                for bin in start_bin..end_bin.min(nyquist) {
                    energy += buffer[bin].norm_sqr();
                }
                band_energies[band] += energy;
            }
            window_count += 1;
        }

        // Average energies across windows
        if window_count > 0 {
            for energy in &mut band_energies {
                *energy /= window_count as f32;
            }
        }

        // Create hash by comparing each band to median energy
        let mut sorted_energies = band_energies.clone();
        sorted_energies.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted_energies[num_bands / 2];

        let mut hash = 0u64;
        for (i, &energy) in band_energies.iter().enumerate() {
            if energy > median {
                hash |= 1 << i;
            }
        }

        Ok(hash)
    }
}

impl Hasher for AudioPHasher {
    fn hash_reader(&self, reader: &mut dyn Read) -> Result<Box<[u8]>, std::io::Error> {
        // Read all data first (needed for audio parsing)
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        // Check if it's likely a WAV file (simple magic byte check)
        if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not a valid WAV file"
            ));
        }

        // Load and process audio
        let samples = self.load_audio_data(&data)?;
        let hash = self.compute_spectral_hash(samples)?;

        // Return hash as 8 bytes
        Ok(hash.to_be_bytes().to_vec().into_boxed_slice())
    }

    fn name(&self) -> &'static str {
        "Audio-pHash"
    }
    
    fn clone_hasher(&self) -> Box<dyn Hasher> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct ImagePHasher;

impl Hasher for ImagePHasher {
    fn hash_reader(&self, reader: &mut dyn Read) -> Result<Box<[u8]>, std::io::Error> {
        
    }

    fn name(&self) -> &'static str {
        "ImagePHasher"
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
    fn test_audio_phash_hasher_empty_input() {
        let hasher = AudioPHasher;
        let mut cursor = Cursor::new(b"");
        let result = hasher.hash_reader(&mut cursor).unwrap();
        assert!(!result.is_empty());
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
