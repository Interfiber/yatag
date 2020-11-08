use crate::console::get_input;
use std::fs::File;
use rand::prelude::*;
use std::path::Path;

pub fn print_people(){
    println!("[1]: Bob");
    println!("[2]: John");
}
pub fn talk_to_john(){
    let tech_repsonses = vec!["Help me defragment my disk","What happens when I delete System32?", "Can I delete the Windows registrey?"];
    let mut rng = rand::thread_rng();
    let linux_response = "What happens when I run 'sudo rm -rf /'?";
    let response = tech_repsonses[rng.gen_range(0, tech_repsonses.len())];
    println!("JOHN: What do you need?");
    println!("MENU: Choose Response");
    println!("  [1]: Give me Advice");
    if rng.gen_range(0, 25) == 15 {
        println!("  [2]: {}", linux_response);
    }else {
        println!("  [2]: {}", response);
    }
    println!("  [3]: Can you give me a Computer?");
    let say = get_input("Enter response number below:");
    if say == "1" {
        println!("JOHN: Dont Know Why you would ever need advice from me but here.");
        println!("JOHN: Never EVER Eat the food called ITEM_ERR");
        println!("JOHN: Its Bad....");
    }else if say == "2" {
        if response == "Help me defragment my disk" {
            println!("JOHN: I forget. I havent done that for a long time.");
        }else if response == "What happens when I delete System32?" {
            println!("JOHN: Say Good bye to your computer.");
        }else if response == "Can I delete the Windows registrey?" {
            println!("JOHN: No. Dont do it.");
        }
    }else if say == "3" {
        println!("JOHN: Here. I have this old Linux Computer. I dont use it. You can have it");
        let _john_computer = File::create(&format!("{}/.yatag-john-computer", std::env::var("HOME").unwrap()));
    }
}
pub fn talk_to_bob(){
    println!("BOB: Well Hello! What do you need?");
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