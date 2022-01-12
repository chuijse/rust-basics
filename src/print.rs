pub fn run() {
    // Print to console
    println!("hello world for printl.rs file");

    // Basic Formatting
    println!("Number: {}", 34);
    println!("{} is from {}", "Brad", "Mass");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named Arguments
    println!("{name} likes to play {acrivity}", name="Jhon", acrivity="Baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 +10);
}