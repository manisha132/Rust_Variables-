fn main() {
    let mut old = String::from("saroj");
    println!("Old name = {}",old);
    change_reference(&mut old);
}
fn change_reference(new: &mut String) { 
    *new = String::from("manisha");
    println!("New name = {}", new);
}