use std::fs;
use std::collections::HashSet;

fn main() {
    // Initial list creation to compare content of 'userstories used.txt'
    let mut list_of_user_stories = HashSet::new();
    let mut used_items = HashSet::new();
    let mut requirements = 0;

    // Creates the user requirements list in the same string format - example "US001"
    for i in 1..=85 {
        let us = if i < 10 {format!("US00{}", i)} else {format!("US0{}", i)};
        // Add each to list of user stories
        list_of_user_stories.insert(us);
    }
    // Takes data from the userstories used.txt file
    let data = fs::read_to_string("userstories used.txt").expect("Failed to read file");
    // Processes data initally line by line
    for line in data.lines() {
        let values: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        requirements += 1;
        for j in values {
            // for each item found it is appended to list
            used_items.insert(j);
        }
    }
    // compare the two lists and any user stories not referenced are a new hashset 
    let remaining_user_requirements: HashSet<String> = list_of_user_stories.difference(&used_items).map(|s| s.to_string()).collect();

    // calculate percentage
    let percentage = ((used_items.len() as f64) / (list_of_user_stories.len() as f64) * 100.0).round();
    println!("The percentage of user stories currently covered by the {} requirements is {}%. {} user stories have not been referenced.", requirements, percentage, list_of_user_stories.len() - used_items.len());
    println!("The user stories that have not been referenced are: {:?}", remaining_user_requirements);
}
