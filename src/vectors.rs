// Vectors - Resizable Vectors

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off value from vectors
    numbers.pop();

    //Print all the Vectors
    println!("{:?}", numbers);
    
    // Get single Val
    println!("Single Value: {}", numbers[0]);

    //Get Vector length
    println!("Vector Length: {}", numbers.len());

    //Vector are stack allocarted
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Number Vec: {:?}", numbers)
}