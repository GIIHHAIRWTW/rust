mod back_of_house;
pub mod front_of_house;

use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
}

fn main() {
    eat_at_restaurant();
}
