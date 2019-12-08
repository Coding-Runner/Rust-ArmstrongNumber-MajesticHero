fn main(){
    
    let num = 153;

    fn is_armstrong_number(num: u32) -> bool {
    
    let numstring = num.to_string();
    numstring.chars().filter_map(|ch| ch.to_digit(10)).map(|digit| digit.pow(numstring.len() as u32)).sum::<u32>()== num
  
}

println!("{} being an armstrong number is {}", num, is_armstrong_number(num));
    
}