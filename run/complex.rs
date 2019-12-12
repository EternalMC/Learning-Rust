use std::io;
extern crate rand;

use rand::distributions::{Distribution, Uniform};

//from a java program I made 

	pub fn main() {
		println!("New Program.");
		
			println!("Welcome to a game.");
		
			print("Please enter your name: ");
				let mut name = String::new();
					io::stdin::().read_line(&mut name).expect(“error: unable to read user input”);
				
				println!("Name: {}", name);

		numgen(); //more errors 
	}
	pub fn numgen() {
		  //Random rand = new Random(); 
			int mut hp = 10;
			//String name;
	      // Generate random integers in range 0 to 50
		let mut rng = rand::thread_rng();
		
    		let rng0 = Uniform::from(1..50);

	      println!("Starting HP: {}", hp);
	      try {
			this.wait(2);
		} catch (InterruptedException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
	      // Print random integers 
	      if rng0 < 20  {
	    	  println!("Dice Roll: {}", rng0);
	    	  hp -= 1;
	    	  println!("HP: " + hp);
	      }
	      else {
	    	  println!("Dice Roll: {}", rng0);
	    	  //println(name + " lives a another day.");
	    	  println!(hp);
	      }
	      
	      System.out.println("Random Integers: "+ rng0); 
	   
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
