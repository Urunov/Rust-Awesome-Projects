
fn march_colours(rgb: (i32, i32, i32)){
    println!("Comparing a colour with {} red, {} blue and green {}: ", rgb.0, rgb.1, rgb.2);

    let new_vec = vec![(rgb.1, "red"), (rgb.2, "green")]; // Put the colour in a vec. Inside are typles with colour names;
    let mut all_at_least_10 = true; // Start with true. We will set it to false if one colour is

    for item in new_vec {
        if item.0 < 10
        {
            all_have_at_least_10 = false; // Now it's false
            println!("Now much {}", item.1); // Add we print the colour name.
        }
    }

    if all_at_least_10 {
        println!("Each colour has at least 10.");
    }
    println!(); // Add one more line;
}