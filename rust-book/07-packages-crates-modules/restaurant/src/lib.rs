pub mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub mod customer {
    pub fn eat_at_restaurant() {
        // absolute path
        crate::front_of_house::hosting::add_to_waitlist();
    
        // relative path
        super::hosting::add_to_waitlist();
    
        // order a breakfast in the summer with rye toast
        let mut meal = super::back_of_house::Breakfast::summer("rye");
        // change our mind about what bread we like
        meal.toast = String::from("wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
    }
}

fn deliver_order() {}


