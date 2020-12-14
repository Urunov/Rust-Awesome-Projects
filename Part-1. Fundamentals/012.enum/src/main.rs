fn main() {
    println!("Enums : Enum is short for enumerations.");

    let time = 14; // it's 8 o'clock
    let skystate = create_skystate(time); // create_skystate returns a ThinkgsInThe Sky
    check_skystate(&skystate); // Give it a reference so it can read the variable skystate


}

enum ThingsInTheSky {
    Sun,
    Starts,
    Sky,
    Cloud,
}

 fn create_skystate(time: i32) -> ThingsInTheSky{
     match time{
         6..=10 => ThingsInTheSky::Sun, // Between 6 and 18 hours we can see the sun
         11..=15 => ThingsInTheSky::Cloud, // Cloud time
         _=> ThingsInTheSky::Starts, // Otherwise, we can see stars
     }
 }

 // With this function we can match against the two choices in ThingsInTheSky.

fn check_skystate(state: &ThingsInTheSky){

    match state {
        ThingsInTheSky::Sun => println!("I can see the sun "),
        ThingsInTheSky::Starts=> println!("I can see start as well"),
        ThingsInTheSky::Cloud=> println!("Cloud we may see now"),
        _ => {}
    }
}

