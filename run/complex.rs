use std::io;

//from a java program I made 

	pub fn main() {
		
		//String name;
		
		int hp = 10;

		
		//Scanner input = new Scanner(System.in);
			System.out.println("Welcome to a game.");
		
			System.out.print("Please enter your name: ");
				//name = input.nextLine();

				
				System.out.println("Name: " + name);
		//input.close();
		numgen(); //more errors 
	}
	pub fn numgen() {
		  //Random rand = new Random(); 
			int hp = 10;
			//String name;
	      // Generate random integers in range 0 to 50 
	      int rand_int1 = rand.nextInt(51); 
	      int rand_int2 = rand.nextInt(51); 
	      System.out.println("Starting HP: " + hp);
	      try {
			this.wait(2);
		} catch (InterruptedException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
	      // Print random integers 
	      if rand_int1 < 20  {
	    	  System.out.println("Dice Roll: "+ rand_int1);
	    	  hp -= 1;
	    	  System.out.println("HP: " + hp);
	      }
	      else {
	    	  System.out.println("Dice Roll: "+ rand_int1);
	    	  //System.out.println(name + " lives a another day.");
	    	  System.out.println(hp);
	      }
	      
	      System.out.println("Random Integers: "+ rand_int2); 
	   
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
