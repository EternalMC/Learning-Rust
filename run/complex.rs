pub fn main() {
  println!("A simple loop");
      for index in 0..5 {
         println!("{}", index);
        
        ifloop();
      }
}
pub fn ifloop() {
    println!("Loop in a if statement:");
let i = 9;
    if i == 9 {
      for index in 0..13 {
         println!("{}", index);
      }
   }   
}
