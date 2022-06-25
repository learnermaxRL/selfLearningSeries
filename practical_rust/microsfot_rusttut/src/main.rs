
// enum random {
//     okay(bool),
//     notOk(bool),
//     randomStruct{a:u8},
//     randomMember
// }

// fn goodbye(message: &str) {
//     println!("\n{}", message);
// }

// fn main() {
//     let formal = "Formal: Good bye.";
//     let casual = "Casual: See you later!";
//     goodbye(formal);
//     goodbye(casual);
// }

// fn main() {
//     let x = random::randomMember;
//     // Define a tuple struct
// #[derive(Debug)]
// struct KeyPress(String, char);

// // Define a classic struct
// #[derive(Debug)]
// struct MouseClick { x: i64, y: i64 }

// // Define the WebEvent enum variants to use the data from the structs
// // and a boolean type for the page Load variant
// #[derive(Debug)]
// enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

// // Instantiate a MouseClick struct and bind the coordinate values
// let click = MouseClick { x: 100, y: 250 };
// println!("Mouse click location: {}, {}", click.x, click.y);
    
// // Instantiate a KeyPress tuple and bind the key values
// let keys = KeyPress(String::from("Ctrl+"), 'N');
// println!("\nKeys pressed: {}{}", keys.0, keys.1);
    
// // Instantiate WebEvent enum variants
// // Set the boolean page Load value to true
// let we_load = WebEvent::WELoad(true);
// // Set the WEClick variant to use the data in the click struct
// let we_click = WebEvent::WEClick(click);
// // Set the WEKeys variant to use the data in the keys tuple
// let we_key = WebEvent::WEKeys(keys);
    
// // Print the values in the WebEvent enum variants
// // Use the {:#?} syntax to display the enum structure and data in a readable form
// println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
//     println!("Hello, world!");


// let formal = "Formal: Good bye.";
//     let casual = "Casual: See you later!";
//     goodbye(formal);
//     goodbye(casual);

// }

// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) ->Car{

    // Use the values of the input arguments
    // All new cars always have zero mileage
    // let car: Car = todo!("Create an instance of a `Car` struct");
    let car = Car {
        color:color,
        transmission:transmission,
        convertible:convertible,
        mileage:0

 
    };
    car
}

fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
}

