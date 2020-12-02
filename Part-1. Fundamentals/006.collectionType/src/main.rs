


fn main() {
    println!("----------Smallest and largest numbers!----------------");

    println!("The smallest i8 is {} and the biggest i8 is {}. " , std::i8::MIN, std::i8::MAX); //std - standard library / i8 - integer (8) -128 ~ 127.
    println!("The smallest u8 is {} and the biggest u8 is {}.", std::u8::MIN, std::u8::MAX);
    println!("The smallest i16 is {} and the biggest i16 is {}. ", std::i16::MIN, std::i16::MAX);
    println!("The smallest u16 is {} and the biggest u16 is {}.", std::u16::MIN, std::u16::MAX);
    println!("The smallest i32 is {} and the biggest i32 is {}.", std::i32::MIN, std::i32::MAX);
    println!("The smallest u32 is {} and the biggest u32 is {}.", std::u32::MIN, std::u32::MAX);
    println!("The smallest i64 is {} and the biggest i64 is {}.", std::i64::MIN, std::i64::MAX);
    println!("The smallest u64 is {} and the biggest u64 is {}.", std::u64::MIN, std::u64::MAX);
    println!("The smallest i128 is {} and the biggest i128 is {}.", std::i128::MIN, std::i128::MAX);
    println!("The smallest u128 is {} and the biggest u128 is {}.", std::u128::MIN, std::u128::MAX);

    println!("--------------- Shadowing -------------------");

    //Shadowing means using let to declare a new variable with the same name as another variable.
    // It looks like mutability, but it is completely different. Shadowing looks like this:


    let  my_number = 8;
    //So is the first my_number destroyed? No, but when we call my_number we now get my_number the f64.
    // And because they are in the same scope block (the same {}), we can't see the first my_number anymore.
    println!("{}", my_number);

    {
        let my_number = 9.2;
        println!("{}", my_number);
          // This is an f64. It is not my_number - it is completely different!
          // Prints 9.2
         // But the shadowed my_number only lives until here.
         // The first my_number is still alive!
    }
    println!("{}", my_number);

    // ---------- Functional shadowing ---------

    let final_number = {

        let y = 10;
        let x = 21;
        let x = multiFunction(x); // shadow with new x : 42

        let x = x + y; // shadow is new x: 52

          x
    };

    println!("The number is now: {}", final_number);



}

fn multiFunction(number: i32) -> i32{
    number*2
}
