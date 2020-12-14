
fn main() {


    println!(" --- VECTOR ---");

    let name1 = String::from("Hash pad");
    let name2 = String::from("Greats");

    let mut my_vec = Vec::new();
    // If we run program now, the compiler will give an error.
    // It does not know the type of vec.

    my_vec.push(name1); // now it knows: it's Vec<String>
    my_vec.push(name2);

    // Vector like a array example

    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Everything is the same as above except we added vec!

    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("
    Three to five: {:?},
    start at two: {:?}
    end at five: {:?}
    everything: {:?}", three_to_five, start_at_two, end_at_five, everything
    );

    // ---------------------------
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is : First item : {:?}
        Second item: {:?}
        Third item: {:?}
        Fourth item: {:?}
        Fifth item: {:?}
        Sixth item: {:?}",

        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    )
}

