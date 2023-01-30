//LearningFunctions Assignment - Smart Home System
//Max Edward | 1/29/23


use std::io; //Use is for bringing items into scope so it can be used, the std::io brings the in and out operations from the standard library.

fn check_temp(_temp: u32) -> String { //This is the "check temp" function for the program. It is used when the check_temp function is called.
    format!("Temperature is set to {}°F.", _temp) //Simply formats the output of the function to state the line.
}

fn check_light(_light_name: &str) -> String { //This is the "check light" function for the program. It is used when the check_light function is called.
    format!("{} light turned on.", _light_name) //Simply formats the output of the function to state the line.
}

fn check_applicance(_appliance: &str) -> String { //This is the "check appliance" function for the program. It is used when the check_appliance function is called.
    format!("{} is currently running and on the network.", _appliance) //Simply formats the output of the function to state the line.
}

fn check_door(_door: &str) -> String { //This is the "check door" function for the program. It is used when the check_door function is called.
    format!("The {} is locked.", _door) //Simply formats the output of the function to state the line.
}

fn security_system(_security: &str) -> String { //This is the "security system" function for the program. It is used when the security_system function is called.
    format!("The security system is currently {}.", _security) //Simply formats the output of the function to state the line.
}

fn main() { //This is the main function of the program.
    //Variables used for the Smart Home System information, will be stored until called.
    let mut _account_name = String::new();
    let _temp: u32 = 75;
    let _temp_result = check_temp(_temp);
    let _light_name = "Living Room".to_owned();
    let _light_result = check_light(&_light_name);
    let _appliance = "Refrigerator".to_owned();
    let _appliance_result = check_applicance(&_appliance);
    let _door = "Front Door".to_owned();
    let _door_result = check_door(&_door);
    let _security = "ENABLED".to_owned();
    let _security_result = security_system(&_security);

    //Getting user input for account signin
    println!("\nHello! Welcome to SmartHomeCentral\n");
    println!("To proceed, your account name.");

    //Get input from user.
    println!("\nAccount Name:");
    io::stdin().read_line(&mut _account_name)
        .expect("Invalid Password.");

    //Ask user what they would like to do, and present with option list.
    println!("\nWelcome back {}", _account_name);
    println!("Enter the name of the smart device you would like to check:");
    println!("OPTIONS:");
    println!("Temperature");
    println!("Light");
    println!("Applicance");
    println!("Door");
    println!("Security\n");

    //Get input from user.
    let mut _option = String::new(); //New String, for temporarily storing user input.
    io::stdin().read_line(&mut _option) //Read user input and store into _option variable.
        .expect("Invalid Selelction"); //Error message in case incorrect value is entered in prior line.
    let _option = _option.trim(); //Trim _option entry from user to remove any extra whitespace not necessary, then restore in variable.

    //Temperature Check Option
    if _option == "Temperature" {
        println!("\n{}", _temp_result);
    }

    //Light Check Option
    if _option == "Light" {
        println!("\n{}", _light_result);
    }

    //Appliance Check Option
    if _option == "Appliance" {
        println!("\n{}", _appliance_result);
    }
    
    //Door Check Option
    if _option == "Door" {
        println!("\n{}", _door_result);
    }

    //Security System Check Option
    if _option == "Security" {
        println!("\n{}", _security_result)
    }
}