use std::io;

fn farenheit_to_celsius(degrees: f32) -> f32 {

    return (degrees - 32.0) * 5.0/9.0;
}

fn celsius_to_farenheit(degrees: f32) -> f32 {

    return (degrees * 9.0/5.0) + 32.0;

}


fn main(){

    let mut option = String::new();
    println!("Do you want to covert \n 1. Fahrenheit to Celsius \n 2. Celsius to Fahrenheit?");

    io::stdin().read_line(&mut option).expect("Failed to read stdin");

    let trimmed = option.trim();
    let x: i32 = trimmed.parse::<i32>().expect("Failed to parse input");

    match x {
        
        1 => {

            let mut option = String::new();
            println!("Input your degrees in Fahrenheit: ");

            io::stdin().read_line(&mut option).expect("Failed to read stdin");
            
            let trimmed = option.trim();
            let degree: f32 = trimmed.parse::<f32>().expect("Failed to parse input");

            let p: f32 = farenheit_to_celsius(degree);

            println!("The value of {} in Celsius is: {:.2}", option, p);
        },

        2 => {

            let mut option = String::new();
            println!("Input your degrees in Celsius: ");

            io::stdin().read_line(&mut option).expect("Failed to read stdin");
            
            let trimmed = option.trim();
            let degree: f32 = trimmed.parse::<f32>().expect("Failed to parse input");
        
            let p: f32 = celsius_to_farenheit(degree);

            println!("The value of {} in Fahrenheit is: {:.2}",option,p);
        },

        _=> println!("Wrong input choice, try again")

    }


}



