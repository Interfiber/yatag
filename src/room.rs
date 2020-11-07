pub fn get_room_items(room: i32, taken_items: Vec<String>){
    if room == 1 {
        if taken_items.contains(&"Bed".to_string()){
            print!("");
        }else {
            println!("  [1]: Bed");
        }
        if taken_items.contains(&"Coffee Maker".to_string()){
            print!("");
        }else {
            println!("  [2]: Coffee Maker");
        }
        if taken_items.contains(&"Toilet".to_string()){
            print!("");
        }else {
            println!("  [3]: Toilet");
        }
    }
    if room == 2 {
        
    }
}
pub fn get_item_name(room: i32, item_id: String) -> String{
    if room == 1 {
        if item_id == "1" {
            return String::from("Bed");
        }else if item_id == "2" {
            return String::from("Coffee Maker")
        }else if item_id == "3" {
            return String::from("Toilet");
        }else {
            println!("No such item in room. Use 'items' to list items in room!");
            return String::from("ITEM_ERR");
        }
    }else {
        println!("No such room exists!");
        return String::from("ROOM_ERR");
    }
} 