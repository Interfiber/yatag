use std::io;
use std::fs::File;
use std::path::Path;
mod room;
static mut ROOM_NUMBER: i32 = 1;
static mut INV_ITEMS: i32 = 0;
pub fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}
fn main() {
    let mut taken_items: Vec<String> = vec![String::from("Bannana Cake")];
    loop {
        let cmd: String = get_input(">");
        if cmd == "exit" {
            std::process::exit(1);
        }
        if cmd == "items" {
            unsafe {
                room::get_room_items(ROOM_NUMBER, taken_items.to_vec());
            }
        }
        if cmd == "take" {
            unsafe {
                let item = get_input("Item Number to take:");
                let item_name = room::get_item_name(ROOM_NUMBER, item);
                taken_items.push(item_name.to_string());
                println!("You have now taken {}!", item_name.to_string());

                INV_ITEMS += 1;
            }
        }
        if cmd == "taken" {
            let mut num = 1;
            for i in &taken_items {
                println!("  [{}]: {}", num, i);
                num += 1;
            }
        }
        if cmd == "eat" {
            unsafe {
                let eat_cmd = std::panic::catch_unwind(|| {
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
            });
            if eat_cmd.is_err(){
                println!("You dont have that item in your inventory!");
            }
        }
        }

    }
}
