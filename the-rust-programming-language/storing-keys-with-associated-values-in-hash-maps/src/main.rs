fn main() {
    //
    // Rust's hash map is a KV store. It uses a hashing function 
    // to determine how how to place the keys and the values into 
    // memory.
    //
    // Hash maps are homogeneous. All the keys have the same type 
    // and all the values have the same type.
    //
    // Hash maps take ownership over the values.
    //
    use std::collections::HashMap;
    
    // creating an empty hash map
    let mut scores = HashMap::new();

    // inserting key value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{} has {:?} scores", team_name, score);

    // accessing values with `for` loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating values
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // `or_insert` returns a mutable reference (&mut V)
        // to the value forthis key.
        let count = map.entry(word).or_insert(0);
        // dereference `count` with `*` in order to assign a 
        // new increment.
        *count += 1;
    }
    println!("{:?}", map);
    
    // creating a hash map from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 1. convert both vectors into iterators
    // 2. zip the vectors to create iterators of tuples 
    //    of (team, initial_score)
    // 3. collect the tuple iterators into a hash map
    let mut scores2: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    // HashMap<_, _> to signal `collect` method to collect into 
    // hash map. Rust can infer the KV types based on the vectors.
}
