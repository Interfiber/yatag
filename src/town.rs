use crate::console::get_input;
pub fn print_people(){
    println!("[1]: Bob");
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
        println!("BOB: Sorry, I dont just carry around cheese all day!");
    }else if say == "3" {
        println!("BOB: Oh! I am Bob I own the store down the street!");
        println!("BOB: Come and visit the store sometime.");
    }
}