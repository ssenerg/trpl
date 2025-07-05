use _017_oop::gui::{Button, Draw, Screen};

fn main() {
    let mut screen = Screen { components: vec![] };
    screen.components.push(Box::new(Button {
        width: 20,
        height: 20,
        label: String::from("B1"),
    }));
    screen.components.push(Box::new(Button {
        width: 120,
        height: 220,
        label: String::from("Button_Name_02"),
    }));
    screen.components.push(Box::new(SelectBox {
        width: 11,
        height: 22,
    }));
    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox\n{}x{}\n", self.width, self.height)
    }
}
