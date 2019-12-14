use std::io; //Gives us access to the standard instruct or string

fn main(){
    
    let mut input = String::new(); //Set a variable "num" as user input.
    
    println!("Hey, type something in to check if it is an armstrong number."); //Prompts user from input
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            
            let num: u32 = input.to_string().parse().unwrap(); //Converts the string back to a u32.
            
            println!("{}", num);
            //fn is_armstrong_number(num: u32) -> bool {
    
    //let numstring = num.to_string();
    //numstring.chars().filter_map(|ch| ch.to_digit(10)).map(|digit| digit.pow(numstring.len() as u32)).sum::<u32>()== num
    //The chars of numstring are filtered and mapped to the absolute value of the digit, then it is raised to the power of the length of numstring, the amount 
    //of digits it has. Collects the sum and changes it to a usize32 and checks if that is equaled to the original number.
  
//} //A function that takes in a usize32 and returns a boolean checking if the number equals each one of the numbers to the power of the amount of digits there are in the number added together.  

//println!("{} being an armstrong number is {}", num, is_armstrong_number(num)); //Prints out if the number is a an armstrong number or not.
    
},
        Err(e) => println!("Oops, something went wrong. {}", e)
    }//This makes the user input "num" and check if it was a success or not. 

}
    
    

    