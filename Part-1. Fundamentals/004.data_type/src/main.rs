fn main() {

    let x = 2.6; // float value
    let y = 4.43;

    let summa = x + y; // summa
    println!("Total sum float = {}", summa);

    let differ = x - y; // difference
    println!("Difference {}", differ);

    let multi = x * y; // multiplication
    println!("Multiple {}", multi);

    let dived = x / y; // division
    println!("Division: {}", dived);

    let reminder = x % y;
    println!("Reminder {}", reminder);


    let tup = (655,443,554);
    let (x, y, z) = tup;
    println!("The value of y is : {}", y);

    let x = (500, 3434.3, 5.4);

    let fivehundered = x.0;
    println!("The value of x is : {}", fivehundered);

    let arr = [1, 3, 5, 4, 2, 1];
    println!("the 3 element {}",arr[2]);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("March is : {}", months[2]);

}
