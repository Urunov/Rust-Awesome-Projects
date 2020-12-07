mod match_colours;

fn main() {


    //------------------------- Loop ------------------------

    println!("=---------The first Loop element=--------");

    let mut counter = 0; // set a counter to 0
    loop{
        counter +=1;
        println!("The counter is now : {}", counter);

        if counter == 5 { // stop when counter = 5
            break;
        }
    }


    //-----------------------------------------------------
    println!("=---------The second Loop element=--------");

    let mut counter1 = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop: ");

    'first_loop: loop {
        // Give the first loop a name
        counter1 +=1;
        println!("The counter is now : {}", counter1);
        if counter1 > 9 {
            // Starts a second loop inside this loop;
            println!("Now entering the second loop inside this loop");
                'second_loop: loop {
                    // now we are inside 'second_loop
                    println!("The second counter is now: {}", counter2);
                    counter2+=1;
                    if counter2 == 3 {
                        break 'first_loop; // Break out of 'first_loop so we can exit the program.
                    }
                }
        }
    }


    //-----------------------------------------------------
    println!("=---------The third Loop element=--------");

    let mut counter3 = 0;

    while counter3 <5 {
        counter3 +=1;
        println!("The counter is now: {}", counter3);

    }
    //------------------------------------------------------

    println!("=---------The Fourth Loop element=--------");

    let mut counter3 = 0;

    while counter3 <5 {
        counter3 +=1;
        println!("The counter is now: {}", counter3);

    }
    //------------------------------------------------------

    println!("=---------The Fifth Loop element=--------");
    //------------------------------------------------------
        for number in 0..5
        {
            println!("The number is :{} ", number);
        }

        for number in 0..=5
        {
            println!("The next number is : {}", number);
        }
    //-------------------------------------------------------
    println!("=---------The Six Loop element=--------");
    for _ in 0..4
    {
        println!("Printing the same thing four time");
    }

    //--------------------------------------------------------
    println!("=---------The seven Loop element=--------");

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);

    // --------------------------------------------------------

    // Leetcode Problem: kthFactor

    let mut n = 3;
    let mut k = 2;
    let mut factor = 0;


    for i in 1..n+1
    {
        if n % i ==0 {
            factor +=1;
        }

        if factor == k {
            println!("{}",i);
            break;
        } else {
            println!("{}", -1);
            break;
        }
    }
}

