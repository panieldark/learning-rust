// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)
}

pub fn main() {
    eat_at_restaurant();
}