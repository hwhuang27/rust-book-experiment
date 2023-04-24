mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use statement needs to be in the same scope when you want to access it
//use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // or we can just use the super keyword to jump out to parent module
        //super::hosting:add_to_waitlist();
        hosting::add_to_waitlist();
    }
}