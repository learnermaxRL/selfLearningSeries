
/*

png files are chunk\
each chunk containsitd own data

png decoder tocan ingore chunk depending on ho 
we capitlaize ou chunk types

rust bytes are represnted by type u8

*/

#![allow(unused_variables)]

// use std::string::ToString
use std::fmt::Display;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;


#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub struct ChunkType {
    content : [u8;4]
}


impl ChunkType {

    pub fn bytes(&self)-> [u8;4] {
        self.content
    }
    pub fn is_valid(&self)-> bool {
       
       for c in self.content {
           println!("{}",c);
           if !((c>=65  && c <=90 ) || (c>=97  && c <=122 )){
               return false;
           }
       }
       return true;
    }

    pub fn is_err(&self)-> bool {
       
        return !self.is_valid()
     }

    pub fn is_critical(&self) -> bool {
       
        println!("{:0b}   is_critical {}",self.content[0],self.content[0]);
        println!("{}",(self.content[0]  >> 5) & 1 );
        if ((self.content[0]  >> 5) & 1 !=0) {
            println!("false");
            false
        }
        else {
            println!("true");
            true
            
        }

    }

    pub fn is_public(&self) -> bool {
        
        println!("{:0b} is_public {}",self.content[1],self.content[1]);
        println!("{}",(self.content[1]  >> 5) & 1);
        if ((self.content[1]  >> 5) & 1 !=0){
            println!("false");
            false
        }
        else {
            println!("true");
            true
        }
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        println!("{:0b} is_reserved_bit_valid {}",self.content[2],self.content[2]);
        println!("{}",(self.content[2]  >> 5) & 1);
        if ((self.content[2]  >> 5) & 1 !=0){
            println!("false");
            false
        }
        else {
            println!("true");
            true
        }

    }

    pub fn is_safe_to_copy(&self) -> bool {
        println!("{:0b} is_safe_to_copy {}",self.content[3],self.content[3]);
        println!("{}",(self.content[3]  >> 5) & 1 );
        if ((self.content[3]  >> 5) & 1 ==0){
            println!("false");
            false
        }
        else {
            println!("true");
            true
        }

    }


}
impl TryFrom<[u8; 4]> for ChunkType {

    type Error = &'static str;    

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error>{
        if false {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(ChunkType{
                content:value
            })
        }
    }
}

impl FromStr for ChunkType {

    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        Ok(ChunkType{
            content:{
                s.as_bytes().try_into().unwrap()
            }
        })

    }
}

// impl ToString for ChunkType {

//     fn to_string(&self) -> String {

//         String::from_utf8(self.content.to_vec()).unwrap()
//     }

// }

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {      
        write!(f, "{}",std::str::from_utf8(&self.content).unwrap().to_string())?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        println!("----s{}----",chunk.is_valid());
        assert!(chunk.is_valid());

        // println
        let chunk = ChunkType::from_str("Ru1t").unwrap();
        println!("----s{}----",chunk.is_valid());
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        println!("{}",chunk);
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}




