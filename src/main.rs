use std::fs;
use std::collections::HashSet;

fn main() {
    // Initial list creation to compare content of 'userstories used.txt'
    let mut list_of_user_stories = HashSet::new();
    //let mut used_items = HashSet::new();
    let mut requirements = 0;

    // Creates the user requirements list in the same string format - example "US001"
    for i in 1..=85 {
        let us = if 1 < 10 {format!("US00{}", i)} else {format!("US0{}", i)};
        // Add each to list of user stories
        list_of_user_stories.insert(us);
    }

    let data = fs::read_to_string("userstories used.txt").expect("Failed to read file");


   println!("{}",data)
}