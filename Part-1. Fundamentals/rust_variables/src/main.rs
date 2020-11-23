

fn main() {

    /**
    The name variable is impossible, because variable is immutable .
    However, key word(mut) possible to use.
    */

    let a1 = 5;
    println!("The value of x is {} ", a1);

    let a2 = 44;
    println!("The second value is {} ", a2);


    /**
        The same name variables using mut
    */

    let mut c = 43;
    let c = 5;

    println!("The first c = {} and y = {}", c, c);

   // println!(" second c = {}", c);

    /**------------------------- */

    let y = c * 3;

    println!("The value of y is {}", y);

    let spaces = "    ";
    let spaces  = spaces.len();
    print!(" Space val: {}", spaces);


}
