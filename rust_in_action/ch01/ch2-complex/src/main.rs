use std::time::{Duration, Instant};                //<1>

fn match_item(){

    let x=[1,2,3,5,7,8,9,56,4,5,7,8];
    let k = 56;
    for it in &x{
        let res = match it {
            56 | 132 => "hit!", 
            _ => "miss"
        };

        if res == "hit!"  {
            println!("{}: {}", it, res);
        };
    }


}

fn ref_sasmple(){
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!("a + a = {}", b);
}


fn main() {
   let mut count = 0;
   let time_limit = Duration::new(1,0);            //<2>
   let start = Instant::now();                     //<3>

   while (Instant::now() - start) < time_limit {   //<4>
       count += 1;
   }
   println!("{}", count);

   match_item();
   ref_sasmple();
}