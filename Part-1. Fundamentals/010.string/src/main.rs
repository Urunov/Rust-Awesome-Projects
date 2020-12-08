fn main() {


    let my_name = "Hamdamboy";
    let my_country = "Uzbekistan";
    let my_home = "Korea";

    let together  = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );

    println!("{}",together);

    // --------------------------------More on references -------------
    // const and static

    let country = String::from("Uzbekistan");
    let ref_one = &country;
    let ref_two = &country;

    println!("-------------------");
    println!("{}", ref_one);
    println!("{}", ref_two);
  // -----------------------------------------------------

   // let countryA = return_str();

  // ------------------------------------------------------

    let mut my_number = 8;
    let num= &mut my_number;
    *num += 10;
    println!("{}", num);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Second_number = triple_reference? {}", second_number == ***triple_reference);


    // --------------------------------
    let country = String::from("Finland");
        prints_country(country.clone()); // make a clone and give it to the function. Only clone goes in,
                                                        // and country is still alive.
        prints_country(country);
    }

    fn prints_country(country_name: String){
        println!("{}", country_name);
    }


  // Reference

    // fn return_str() -> &str
    //   {
    //       let countryA = String::from("Finland");
    //       let countryB = String = &countryA;
    //       countryB;
    //   }
