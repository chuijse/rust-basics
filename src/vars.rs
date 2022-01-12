// Variables hold primitive data or refernces to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
   let name = "Cristian";
   let mut age = 34; // mut for mutable variable
   println!("My name is {} and I am {}", name, age);
   age = 35;
   println!("My name is {} and I am {}", name, age);

   //Define constant
   const ID: i32 = 001;
   println!("ID: {}", ID);

   //Assign Multiple vars
   let (my_name, my_age) = ("Cristian", 34);
   println!("{} is {}", my_name, my_age);
}