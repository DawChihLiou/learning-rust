fn main() {
    let config_max = Some(3u8);
    //match config_max {
    //    Some(max) => println!("The maximum is configured to be {}", max),
    //    _ => (),
    //}
    
    // the match block above can be replaced with if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
}
