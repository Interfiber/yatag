use crate::console::get_input;
use std::fs::File;
use std::path::Path;

pub fn print_people(){
    println!("[1]: Bob");
    println!("[2]: John");
}
pub fn talk_to_john(){
    
}
pub fn talk_to_bob(){
    println!("BOB: Well Hello What do you need?");
    println!("MENU: Choose Response");
    println!("  [1]: I need you to go away");
    println!("  [2]: I need you to give me cheese");
    println!("  [3]: Who Are You?");
    let say = get_input("Enter response number below:");
    if say == "1" {
        println!("BOB: Maybe next time be nicer. Just a tip.");
    }else if say == "2" {
        if Path::new(&format!("{}/.yatag-bob-has-no-cheese", std::env::var("HOME").unwrap())).exists(){
            println!("BOB: I will drop off cheese at your house today!");
            std::fs::remove_file(&format!("{}/.yatag-bob-has-no-cheese", std::env::var("HOME").unwrap())).expect("Failed to remove cache!");
            let _bob_needs_to_deliver_cheese = File::create(&format!("{}/.yatag-bob-will-deliver-cheese", std::env::var("HOME").unwrap()));
        }else {
            println!("BOB: Sorry, I dont just carry around cheese all day!");
            let _bob_has_no_chesse_cache = File::create(&format!("{}/.yatag-bob-has-no-cheese", std::env::var("HOME").unwrap()));
        }
    }else if say == "3" {
        println!("BOB: Oh! I am Bob I own the store down the street!");
        println!("BOB: Come and visit the store sometime.");
    }
}