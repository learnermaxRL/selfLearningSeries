
/*

png files are chunk\
each chunk containsitd own data

png decoder tocan ingore chunk depending on ho 
we capitlaize ou chunk types

rust bytes are represnted by type u8

*/

struct Chunk {
    length : u32,
    chunkType:&[u8;4],
    chunkData:Vec<u8>,
    CRC:&[u8;4],

}