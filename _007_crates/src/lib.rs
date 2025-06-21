struct Sth {}

mod flower {
    pub struct Flower {}
    fn sth() -> super::Sth {
        super::Sth{}
    }
}
mod garden;
pub mod rock;
mod front_of_house {
    pub mod hosting {
        fn add_to_wait_list_pr() {}
        pub fn add_to_wait_list_pub() {
            let _ = super::Private{field: String::new()};
            add_to_wait_list_pr();
        }

        pub mod chef {
            use super::add_to_wait_list_pr;
            pub fn fix_order() {
                add_to_wait_list_pr();
            }
        }
    }
    struct Private {
        field: String,
    }
}

pub mod a_test {

    #[derive(Debug)]
    pub struct Jeezez {
        pub field: String,
        booled: bool,
    }

    impl Jeezez {
        pub fn new() -> Self {
            Self {
                field: String::new(),
                booled: false,
            }
        }
    }
    // if jeezez were private code would not compile
    pub fn sutun() -> Jeezez {
        Jeezez {
            field: String::from("damn girl"),
            booled: true,
        }
    }
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_wait_list_pub();
}

pub fn chef_fix_order() {
    front_of_house::hosting::chef::fix_order();
}