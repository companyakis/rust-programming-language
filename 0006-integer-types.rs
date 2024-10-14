fn main() {

    println!("Max u16: {}", u16::MAX);

    println!("Min u32: {}", u32::MIN);

    println!("Min i64: {}", i64::MIN);

// Max u16: 65535
// Min u32: 0
// Min i64: -9223372036854775808
    
}

/*

Length	Signed	Unsigned

8-bit  	i8    	u8

16-bit	i16	    u16

32-bit	i32   	u32

64-bit	i64   	u64

128-bit	i128  	u128

arch	  isize	  usize

size and usize types depend on the architecture of the computer your program is running on, 
which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture

*/

