fn main(){
    
    let num = 153; //Set a variable "num" as 153, an armstrong number.

    fn is_armstrong_number(num: u32) -> bool {
    
    let numstring = num.to_string();
    numstring.chars().filter_map(|ch| ch.to_digit(10)).map(|digit| digit.pow(numstring.len() as u32)).sum::<u32>()== num
    //The chars of numstring are filtered and mapped to the absolute value of the digit, then it is raised to the power of the length of numstring, the amount 
    //of digits it has. Collects the sum and changes it to a usize32 and checks if that is equaled to the original number.
  
} //A function that takes in a usize32 and returns a boolean checking if the number equals each one of the numbers to the power of the amount of digits there are in the number added together.  

println!("{} being an armstrong number is {}", num, is_armstrong_number(num)); //Prints out if the number is a an armstrong number or not.
    
}
