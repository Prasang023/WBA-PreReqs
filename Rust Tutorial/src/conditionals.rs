pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;
  
    if age >= 21 && check_id || knows_person_of_age {
      println!("Bartender: Kya lenge aap?");
    } else if age < 21 && check_id {
      println!("Bartender: Sorry, Nikal");
    } else {
      println!("Bartender: ID?");
    }
  
    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)
  }
  