
pub fn run() {
    println!("Rust is awesome.");
    
    let i = 16;
    println!("{}", i);
    
    let a = 'a';
    println!("{}", a);
    
    println!("{}, {}", i, a);
    
    let look = "Look";
    let what = "what";
    let i = "I";
    let can = "can";
    let doin = "do in";
    let rust = "rust";
    
    println!("{}, {}, {}, {}, {}, {}", look, what, i, can, doin, rust);
}
pub fn math() {
let x = 2;
let y = 2;

 let z = x + y;

   println!("Adding 2 + 2 = {}", z);
   }
pub fn loops() {
  println!("A simple loop");
      for index in 0..2 {
         println!("{}", index);
      }
}
pub fn ifloop() {
    println!("Loop in a if statement");
let i = 9;
    if i == 9 {
      for index in 0..11 {
         println!("{}", index);
      }
   }   
}