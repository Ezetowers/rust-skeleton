use rust_skeleton::{Screen, SelectBox, Button};

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