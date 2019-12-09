fn main(){
    
    let num = 153; //Set a variable "num" as 153, an armstrong number.

    fn is_armstrong_number(num: u32) -> bool {
    
    let numstring = num.to_string();
    numstring.chars().filter_map(|ch| ch.to_digit(10)).map(|digit| digit.pow(numstring.len() as u32)).sum::<u32>()== num
  
} //A function that takes in a usize32 and returns a boolean checking if the number equals each one of the numbers to the power of the amount of digits there are in the number added together.  

println!("{} being an armstrong number is {}", num, is_armstrong_number(num)); //Prints out if the number is a an armstrong number or not.
    
}
