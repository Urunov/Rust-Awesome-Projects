fn main() {
    println!("Pointer");

    let number = 15;
    let single_reference = &number;
    let double_reference = &single_reference;
    let five_reference = &&&&&number;

    println!("five: {}", five_reference);

    println!("--- Line of ----");
    println!("\t Start with a tab, as book and youtube \n course  new progress.");

    println!("Here is more information \\n one and two \\t processing "); // just cancel of the /n and /t

    println!("-------------------------------------");

    println!(" ------- Elements  ---------");
    println!("{:?}", b"This will look like numbers");



    println!("");
    let title = "Today's news in Korea ";
    println!("{:-^38}", title);
    let bar = "|";
    println!("{: <10}{: >12} {: >15}", bar, bar, bar); // no variable name, pad with space 15 characters each
    let a = "SEOUL";
    let b= "TOKYO";
    let c = "Tashkent";
    println!("{city1:-<12}{city2:->12}{city3:->15}", city1 = a, city2=b, city3 = c); // variable name city1 and city2.

    println!("");
    println!("------------String --------------------");
    let name = "Hero";
    let lastName = String::from("Urunov");
    println!("My name is {}",name);
    println!("My surname is {}", lastName);

}


