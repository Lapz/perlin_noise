//! A simple perlin noise implementation in Rust
//!
//! # Example
//! ```rust
//! extern crate perlin_noise as perlin;
//! use perlin::PerlinNoise;
//!
//! fn main() {
//!   let perlin = PerlinNoise::new();
//!   println!("{}",perlin.get(132.2));
//!   println!("{}",perlin.get2d([12.0,32.0]));
//!   println!("{}",perlin.get3d([12.0,32.0,25.0]));
//! }
//! ```
//!  
//!
//!

extern crate rand;

mod perlin;

pub use perlin::PerlinNoise;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::PerlinNoise;

        let perlin = PerlinNoise::new();

        assert_eq!(perlin.get(0.0), 0.46875);
    }
}
