use std::fs::File;
use std::path::Path;
mod room;
mod console;
mod town;
use console::get_input;
static mut ROOM_NUMBER: i32 = 1;
static mut INV_ITEMS: i32 = 0;

fn main() {
    let mut taken_items: Vec<String> = vec![String::from("Bannana Cake")];
    // Does bob need to deliver cheese?
    loop {
        if Path::new(&format!("{}/.yatag-bob-will-deliver-cheese", std::env::var("HOME").unwrap())).exists(){
            println!("Alert: Bob Delivered Cheese. Check for it using 'taken'");
            taken_items.push(String::from("Cheese"));
            std::fs::remove_file(&format!("{}/.yatag-bob-will-deliver-cheese", std::env::var("HOME").unwrap())).expect("Failed to remove cache!");
        }
        // Every loop is 1 second
        let cmd: String = get_input(">");
        if cmd == "exit" {
            std::process::exit(1);
        }
        else if cmd == "items" {
            unsafe {
                room::get_room_items(ROOM_NUMBER, taken_items.to_vec());
            }
        }
        else if cmd == "take" {
            unsafe {
                let item = get_input("Item Number to take:");
                let item_name = room::get_item_name(ROOM_NUMBER, item);
                taken_items.push(item_name.to_string());
                println!("You have now taken {}!", item_name.to_string());

                INV_ITEMS += 1;
            }
        }
        else if cmd == "taken" {
            let mut num = 1;
            for i in &taken_items {
                println!("  [{}]: {}", num, i);
                num += 1;
            }
        }
        else if cmd == "eat" {
            unsafe {
                let item = get_input("Enter Item Number to Eat:");
                let item_name = &taken_items[item.parse::<i32>().unwrap() as usize - 1];
                if ROOM_NUMBER == 1 {
                    if item_name == "Bed" {
                        println!("You eat a bed. That was a bad idea");
                        println!("You died!");
                        std::process::exit(1);
                    }
                    if item_name == "Coffee Maker" {
                        println!("You eat an entire Coffee Maker. Maybe you should have made coffee.");
                        println!("You Died!");
                        std::process::exit(1);
                    }
                    if item_name == "Toilet" {
                        println!("You eat the toilet. While Bob your neighbor is on it.");
                        println!("Next Time eat the Bannana Cake.");
                        println!("You Died!");
                        let _toilet_eat = File::create(&format!("{}/.yatag-toilet-ate", std::env::var("HOME").unwrap()));

                        std::process::exit(1);
                    }
                    if item_name == "Bannana Cake" {
                        println!("Yum! The Bannana Cake was amazing!");
                        if Path::new(&format!("{}/.yatag-toilet-ate", std::env::var("HOME").unwrap())).exists(){
                             println!("See what I mean! Bannana Cake Tastes Better that Toilet with Bob!");
                        }
                    }
                }
                // Items that user can eat from anywhere
                if item_name == "Cheese" {
                    println!("You ate cheese. Now you have bad breath");
                }
                // Remove item from inventory
                taken_items.remove(item.parse::<i32>().unwrap() as usize - 1);
        }
        }
        else if cmd == "help" {
            println!("Commands:");
            println!("eat : Eat item");
            println!("taken : Print Taken Items");
            println!("take : Take Item from current room");
            println!("items : Print Items in room");
            println!("talk : Talk to someone");
            println!("people : Get List of people in town");
        }
        else if cmd == "people" {
            town::print_people();
        }
        else if cmd == "clear" {
            println!("Clearing your screen...");
            // Print Escape code to clear the terminal screen
            print!("\x1B[2J\x1B[1;1H");
        }
        else if cmd == "talk" {
            let person = get_input("Enter person number to talk to:");
            if person == "1" {
                println!("Talking to bob...");
                town::talk_to_bob();
            }
            if person == "2" {
                println!("Talking to John...");
                town::talk_to_john();
            }
        }else {
            println!("I have no idea what you just said");
        }
        
    

    }
}
