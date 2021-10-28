fn main() {
    let res = String::from("manisha");
    
    // gfg is passed as reference
    let length = calculate_length(&res); 
  
    // gfg goes out of scope
    println!("The length of '{}' is {}.", res, length);
} 
  
fn calculate_length(res_len: &String) -> usize { 
    
    // gfg_len variable expecting string reference
    // returning length
    res_len.len() 
}