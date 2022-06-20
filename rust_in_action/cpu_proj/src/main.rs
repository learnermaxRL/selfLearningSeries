/*
in computer bits 

nibble - 4 bits  range in bin - 0-15  in hex - 0-0xF
bytes - 8 bits  bin - 0-255 0 -oxFF
each number in hex represents 4 bits
so 0X f000 -  

bitmasking

We can turn a bit on by a bitwise OR ( | ) with 1 in the relevant position.
We can test whether a bit is set by a bitwise AND (&) with 1 in the relevant position.
We can turn a bit off by a bitwise AND (&) with NOT (∼) 1 in the relevant position.
We can toggle a bit by a bitwise XOR (∧
) with 1 in the relevant position.


*/

//Leaving this book as its giving out every code and its hard not look at solutions
// and do it myself aso tthe instructtions are not proper fortob esable tto code independelly

extern crate hex;
struct CPU {

    position_in_mem :usize;
    regisers:[u8;16]; // 16 registers so we can use a single hex code to access a regiterr
    memory:[u8;0x1000] //4096 bytes of ram

}


impl CPU {

    fn read_opcode(&self){
        let p = self.position_in_mem;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2

    }


    fn parse_opcode(opcode:u16)->(u8,u8,u8,u8){
       
        let c = ((opcode_bin & 0xF000)>>12);
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4; 
        let d = (opcode & 0x000F) >> 0;

        return (c,x,y,d)

    }
    fn add_xy(&mut self,x:u8,y:u8){

    }

    fn run (&self){
        loop {

             let (c,x,y,d) = parse_opcode(self.curren_op);

             match(c,x,y,d){
                ()

             }


        }

    }

}


fn main() {
    println!("Hello, world!");
}
