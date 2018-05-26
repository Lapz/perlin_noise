 A simple perlin noise implementation in Rust
 
 [![Build Status](https://travis-ci.org/Lapz/perlin_noise.svg?branch=master)](https://travis-ci.org/Lapz/perlin_noise)
 
 # Example
 ```rust 
 extern crate perlin_noise as perlin;
 use perlin::PerlinNoise
 
 fn main() {
  let perlin = PerlinNoise::new();
  
   println!("{}",perlin.get(132.2));
   println!("{}",perlin.get2d([12.0,32.0]));
   println!("{}",perlin.get3d([12.0,32.0,25.0]));
 }
 ```