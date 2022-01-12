// Conditionals - Used to check the condition of somthing and act on the result

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    //Shorthand if
    let if_of_age = if age >= 21 {true} else {false};
    println!("Is Of Age: {}", if_of_age);


}