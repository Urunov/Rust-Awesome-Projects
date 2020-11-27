fn main() {
    println!("Control Flow");

    let number = 10;

    if number % 4 ==0 {
        println!("number is divisible by 4");
    } else if number % 3 ==0 {
        println!("number is divisible by 3");
    } else if number % 2 ==0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2");
    }

    //------------------ String --------------------------
    let mut  s = String::from("Processing");

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s);

    //--------------------------------------------

    let s1 = String::from("Rust");
    let s2 = s1.clone();  // clone String value

    println!("s1 = {}, s2={}", s1, s2);

   // ------------------------------------------------

    let s = String::from("Processing in Rust language");

    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    //---------------------------------------------------

    let s1 = String::from("Brovo");

    let length = s1.len();

    println!("Length: {}", length );

    // ---------------------------------------------------


}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_string: i32){
    println!("{}", some_string);
}
