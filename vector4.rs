fn value(n:Option<&char>) 
{ 
    match n 
    { 
        Some(n)=>println!("Element of vector {}",n), 
        None=>println!("None"), 
    } 
}
 
fn main() {
 
    let v = vec!['G','E','E','K','S'];
 
    // here index is the non negative value which is
    // smaller than the size of the vector
    let index: usize = 3;
 
    // getting value at given index value
    let ch: Option<&char> = v.get(index);
    value(ch);
}