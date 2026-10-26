use std::collections::HashMap;

fn creating_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}


fn access_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // get returns an Option<&V>, so we need to use match or unwrap to get the value
    match score {
        Some(score) => println!("The score of {} is {}", team_name, score),
        None => println!("The team {} does not exist", team_name),
    }


    // or via for loop
    scores.insert(String::from("Green"), 25);
    for (key, value) in &scores {   // &score is the reference to the hashmap, so we can iterate over the key-value pairs without taking ownership of the hashmap, so we can still use the hashmap after the for loop
        println!("{}: {}", key, value);  // rust does auto * dereferencing, so we can use the key and value without dereferencing them, but we can also use *key and *value to dereference them, but it is not necessary because rust does auto dereferencing
    }
    // Iterating over &scores yields references, so this does not take ownership
    // of the map or its entries.
}


fn managin_ownership_hashmap() {
    let mut scores = HashMap::new();  // needs to be mutable because we are going to insert key-value pairs into the hashmap

    let team_name = String::from("Blue");
    let team_score = 10;

    scores.insert(team_name, team_score); // team_name is moved into the hashmap, so we cannot use it after this line
    // println!("{}", team_name); // this will not work because team_name has been moved into the hashmap, so we cannot use it after this line

    let team_name = String::from("Yellow");
    let team_score = 50;

    scores.insert(team_name.clone(), team_score); // team_name is cloned and moved into the hashmap, so we can still use it after this line
    println!("{}", team_name); // this will work because team_name has been cloned and moved into the hashmap, so we can still use it after this line

}



fn updating_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.insert(String::from("Blue"), 25); // this will overwrite the previous value of Blue, so the score of Blue will be 25 now
    println!("{:?}", scores); // print the hashmap

    // only insert a value if the key does not exist
    scores.entry(String::from("Green")).or_insert(30); // this will insert the key-value pair Green:30 into the hashmap because Green does not exist in the hashmap
    scores.entry(String::from("Blue")).or_insert(50); // this will not insert the key-value pair Blue:50 into the hashmap because Blue already exists in the hashmap, so it will not overwrite the previous value of Blue, which is 25
    println!("{:?}", scores); // print the hashmap

    // update a value based on the old value
    let team_name = String::from("Blue");
    let score = scores.entry(team_name).or_insert(0); // this will return a mutable reference to the value of Blue, so we can update it based on its old value
    *score += 10; // dereference the mutable reference and add 10 to it, so the score of Blue will be 35 now
    println!("{:?}", scores); // print the hashmap
}


fn hashmap_other_key_int() {
    let mut scores = HashMap::new();

    scores.insert(1, String::from("Blue"));
    scores.insert(2, String::from("Yellow"));

    println!("{:?}", scores); // print the hashmap
}


fn hashmap_key_struct() {
    #[derive(Hash, Eq, PartialEq, Debug)] // we need to derive Hash, Eq and PartialEq traits for the struct to be used as a key in the hashmap
    struct Team {
        name: String,
        score: u32,
    }  // derive makes the struct implement the Hash, Eq and PartialEq traits, so we can use it as a key in the hashmap

    let mut scores = HashMap::new();

    let team1 = Team {
        name: String::from("Blue"),
        score: 10,
    };

    let team2 = Team {
        name: String::from("Yellow"),
        score: 50,
    };

    scores.insert(team1, String::from("Team 1"));
    scores.insert(team2, String::from("Team 2"));

    println!("{:?}", scores); // print the hashmap
}


// A Hashmap is like a dict in Python, it is a collection of key-value pairs, where each key is unique and maps to a value. In Rust, Hashmaps are implemented using the HashMap struct from the std::collections module. The keys and values can be of any type that implements the Eq and Hash traits. The HashMap struct provides methods for inserting, accessing, and removing key-value pairs, as well as iterating over the keys and values.
// Hashmap key value pairs are stored in a random order, so the order of the keys and values is not guaranteed to be the same as the order in which they were inserted. If you need to maintain the order of the keys and values, you can use a BTreeMap instead of a HashMap. A BTreeMap is a sorted map that maintains the order of the keys and values based on their natural ordering or a custom ordering defined by a comparator function. However, BTreeMaps are generally slower than HashMaps for large datasets, so you should choose the appropriate data structure based on your specific use case and performance requirements.
fn main() {
    creating_hashmap();
    access_hashmap();
    managin_ownership_hashmap();
    updating_hashmap();
    hashmap_other_key_int();
    hashmap_key_struct();
}


// a hashmap key can be any type that implements the Eq and Hash traits, 
//which means that the key must be comparable for equality and must have a hash value. 
// The most common types used as keys in a hashmap are strings, integers, and tuples of these types. 
// However, you can also use custom types as keys in a hashmap, as long as they implement the Eq and Hash traits. 
//To implement the Eq trait for a custom type, you need to implement the PartialEq trait and define the eq method, which compares two instances of the type for equality. To implement the Hash trait for a custom type, you need to implement the hash method, which computes a hash value for an instance of the type based on its fields. You can use the derive attribute to automatically generate implementations of the Eq and Hash traits for your custom type, as long as all of its fields also implement these traits.

// Hashmap can be a struct as key