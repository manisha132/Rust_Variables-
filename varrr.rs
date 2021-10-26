fn main() {
    let a = 5;

    let a = a + 1;

    {
        let a = a* 2;
        println!("The value of a in the inner scope is: {}", a);
    }

    println!("The value of a is: {}", a);
}