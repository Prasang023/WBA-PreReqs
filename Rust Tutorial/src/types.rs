/*
Primitive Types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
bool
char
Tuples
Arrays
*/

pub fn run() {
    // Default is "i32"
    let x = 1;
  
    // Default is "f64"
    let y = 2.5;
  
    // Add explicit type
    let z: i64 = 4664675365574;
  
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
  
    let is_active= true;
  
    let is_greater: bool = 10 < 5;
  
    let a1 = 'a';
    let face = '\u{1F600}';
  
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
  }