extern crate piston_window;

use piston_window::*;

fn main() {

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [620, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                        [0.0, 0.0, 100.0, 100.0],
                        context.transform,
                        graphics);
        });
    }

    let x = 1; // integer
    let y = 2.2; // float
    let nick = "nick is a string"; // character type
    let boolean = false; // or false boolean

    let dynamic_math = 8 * 8;

    let my_array = [1, 2, 3, 4, 5, 6]; // array

    let my_tuple = (5, 5.5, "5"); // tuple

    let (dynamicx, dynamicy, dynamicz) = my_tuple; // using tuple to create variables

    println!("The value of x is {}", dynamicy);
    hello_function("Second line");

    add(8, 43);
}

fn hello_function(message: &str){
    println!("{}", message);
}

fn add(x: i8, y: i8) {
    println!("{}", x+y);
}

