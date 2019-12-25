use std::io;

fn main(){
    
    loop {


    println!("Hey, type something in to check if it is an armstrong number."); 

    let mut str_number = String::new();

        io::stdin().read_line(&mut str_number)
            .expect("Failed to read line");

        str_number = str_number.trim().to_string();
        
        let number: u32 = match str_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, try again.");
                break
            }
        };
    let numstring = number.to_string(); 
    println!("{} being an armstrong number is {}", str_number, numstring.chars().filter_map(|ch| ch.to_digit(10)).map(|digit| digit.pow(numstring.len() as u32)).sum::<u32>()== number); 
    }
}

    