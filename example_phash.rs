// Example pure Rust image pHash implementation
use image::{DynamicImage, GrayImage, imageops};

pub fn image_phash(img: DynamicImage) -> Result<u64, Box<dyn std::error::Error>> {
    // 1. Convert to grayscale and resize to 8x8
    let gray = img.into_luma8();
    let resized = imageops::resize(&gray, 8, 8, imageops::FilterType::Lanczos3);
    
    // 2. Calculate average pixel value
    let total: u32 = resized.pixels().map(|p| p.0[0] as u32).sum();
    let average = total / 64;
    
    // 3. Create hash
    let mut hash = 0u64;
    for (i, pixel) in resized.pixels().enumerate() {
        if pixel.0[0] as u32 > average {
            hash |= 1 << i;
        }
    }
    
    Ok(hash)
}

// Compare two hashes (Hamming distance)
pub fn hash_distance(hash1: u64, hash2: u64) -> u32 {
    (hash1 ^ hash2).count_ones()
}