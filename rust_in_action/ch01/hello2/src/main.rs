use std::convert::TryInto;



// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//  fn greet_world() {
//     println!("Hello, world!");
//     let southern_germany = "Grüß Gott!";
//      let japan = "ハロー・ワールド";
//      let regions = [southern_germany, japan];
//      for region in regions.iter() {
//      println!("{}", &region);
//      }
//     }

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// fn rand_fn(){
    
//     let penguin_data = "\
//     3 common name,length (cm)
//     4 Little penguin,33
//     5 Yellow-eyed penguin,65
//     6 Fiordland penguin,60
//     7 Invalid,data
//     8 ";

//     let records = penguin_data.lines();
//     for (i,record) in records.enumerate(){

//         if (i!=0) && record.trim().len()>0{

//             let fields:Vec<_> = record.split(',').map(|x| x.trim()).collect();
//             let name = fields[0];
//             if let Ok(length) = fields[1].parse::<f32>() { 
//                 println!("{}, {}cm", name, length);
//             }
            
//         }
         
//     }


// }

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// #[derive(Debug)] 
// enum Cereal {
//     Barlet,Millet,Rice
// }

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// fn illus3(){
//     let x = 20_i32;
//     let y = 30_i32;
//     let three = 0b11;
//     let thirty = 0o36; 
//     let three_hundred = 0x12C; 
//     println!("base 10: {} {} {}", three, thirty, three_hundred);
//     println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
//     println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);


// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// }
// fn illus4() {
//     let a: i32 = 10;
//     let b: u16 = 100;
//     let c = 300_i32;
//     if a < (b as i32) {
//     println!("Ten is less than one hundred.");
//     let c_:i8 = c.try_into() .unwrap();
//     println!("{}",c_ );
//     }
//     }

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
fn illus5(){
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2); 


}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// fn add (x:i32 , y:i32 )->i32 {
//     x+y
// }

fn main(){
    // greet_world();
 
    // rand_fn();
    illus5();
}