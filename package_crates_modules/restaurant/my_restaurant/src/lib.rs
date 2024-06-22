// mod key word is used to create mdoules
/* mod front_of_house { 
    mod hosting {
        fn add_to_waitlist(){}
        fn eat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
} */

////////////////////////////////////////////////////////////////////////////////////////////////////


/* 
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}  // pub make it publice so that we can use it outside the modules
    }
}


pub fn eat_at_restaurant(){
    //Absolue path

    crate::front_of_house::hosting::add_to_waitlist();  // here we are call the add_to_wwailist from fornt_of_house

    //relative path

    front_of_house::hosting::add_to_waitlist();

} */


////////////////////////////////////////////////////////////////////////////////////////////////////

/* fn serve_order(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();  // we are calling the server_order by the use of relative path with super which allow as to referenec the parent modules
    }

    fn cook_order(){}
} */

////////////////////////////////////////////////////////////////////////////////////////////////////


// modules with struct


/* mod back_of_house{
    pub struct Breakfast {
       pub toast : String,
        seasonal_fruit : String,
    }

    impl Breakfast {
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal=
    back_of_house::Breakfast::summer("grild");

    meal.toast = String::from("wheat")
} */


/////////////////////////////////////////////////////////////////////////////////////////////////

// we are importing the modules form another file

mod front_of_house; // this help as to import the module


pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}