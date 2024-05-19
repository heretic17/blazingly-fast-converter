pub fn pound_kg() {
    //lbs * (0.45359237 / 1lbs)
    //1 kg = 2.2 lb
    println!("Select Method");
    println!("1. Pound to Kg");
    println!("2. Kg to Pound");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read the line");

    let parse_choice: i8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_)=> {
            println!("Invalid input. Please enter a number");
            return;
        }
    };

    println!("Enter the value");

    let mut value = String::new();
    let mut parse_value:Option<f32> = None;
    std::io::stdin().read_line(&mut value).unwrap();
    parse_value = value.trim().parse().ok();

    match parse_choice {
        1 => {
            if let Some(val) = parse_value {
                let pound_to_kg = val * (0.45359237 / 1.0);
                println!("{} Pound is {} Kg", val, pound_to_kg);
            } else {
                println!("Failed to parse the input as a number.");
            }
        }
        2 => {
            if let Some(val) = parse_value {
                let kg_to_pound = val * 2.2;
                println!("{} Kg is {} Pound", val, kg_to_pound);
            } else {
                println!("Failed to parse the input as a number.");
            }
        }
        _ => {
            println!("Invalid choice");
        }
    }
}

pub fn inch_cm() {

    println!("Select Method");
    println!("1. Inch to cm");
    println!("2. Cm to inch");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read the line");

    let parse_choice_inch_cm: i8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_)=> {
            println!("Invalid input. Please enter a number");
            return;
        }
    };

    println!("Enter the value");

    let mut value = String::new();
    let mut parse_value:Option<f32> = None;
    std::io::stdin().read_line(&mut value).unwrap();
    parse_value = value.trim().parse().ok();
    //1 cm = 0.393701 inches
    //2.54
    match parse_choice_inch_cm {
        1 => {
            if let Some(val) = parse_value {
                let pound_to_kg = val * 2.54;
                println!("{} inch is {} cm", val, pound_to_kg);
            } else {
                println!("Failed to parse the input as a number.");
            }
        }
        2 => {
            if let Some(val) = parse_value {
                let kg_to_pound = val * 0.393701;
                println!("{} cm is {} inches", val, kg_to_pound);
            } else {
                println!("Failed to parse the input as a number.");
            }
        }
        _ => {
            println!("Invalid choice");
        }
    }
}