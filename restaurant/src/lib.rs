mod front_of_house ;//semicolon tells rust to take contents of module from another file

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist(); 
}