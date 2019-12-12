use std::io;
extern crate rand;

use rand::Rng;

//from a java program I made 

	pub fn main() {
		println!("New Program.");
		
			println!("Welcome to a game.");
		
			print!("Please enter your name: ");
				let mut name = String::new();
				match io::stdin().read_line(&mut name) {
    					Ok(n) => {
						println!("{} bytes read", n);
      						  println!("Name: {}", name);
   					 }
   				 Err(error) => println!("error: {}", error),
				}				

		numgen(); //more errors 
	}
	pub fn numgen() {
		  //Random rand = new Random(); 
			let mut hp: i8 = 10;
			//String name;
	      // Generate random integers in range 0 to 50
    		let rng0 = rand::thread_rng().gen_range(0, 51);
    		let rng1 = rand::thread_rng().gen_range(0, 51);
		

	      println!("Starting HP: {}", hp);

	      // Print random integers 
	      if rng0 < 20  {
	    	  println!("Dice Roll: {}", rng0);
	    	  hp -= 1;
	    	  println!("HP: " + hp);
	      }
	      else {
	    	  println!("Dice Roll: {}", rng0);
	    	  //println!(name + " lives a another day.");
	    	  println!(hp);
	      }
	      
	      println!("Random Integers: "+ rng1); 
	   
	      //do not place anything after this
}

/*
use std::io;
use std::io::*;
fn main(){
let mut input = String::new();
io::stdin::().read_line(&mut input).expect(“error: unable to read user input”);
println!("{}",input);
}
*/
