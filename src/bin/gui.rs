pub trait Draw {
    fn draw(&self);
    fn description(&self) ->  &'static str;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            println!("Drawing object: {}", component.description());
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }

    fn description(&self) ->  &'static str {
        "Button"
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }

    fn description(&self) ->  &'static str {
        "SelectBox"
    }
}

/*------------------------------------------------------------------------*/

// This is the equivalent of defining a vector of elements
// of the same type in C++. Since this code is solved at
// compile time, all the elements of the components vector
// must be of the same type. The not commented definition of the Screen struct
// let the user define diferent objects in the vector that are compliant with
// the trait since the object type is determined at runtime. As always, 
// figure out things at runtime is slower (v-table)
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}