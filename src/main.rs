
use std::fs;

// virtual machine config
const MEMORY_SIZE :usize = 65536;
const STACK_SIZE :usize = 4096;
const REGISTER_NUM :usize = 8;

const REG_PC :usize = 0;    // program counter
const REG_SP :usize = 1;    //stack pointer
const REG_A :usize = 2;     // 2 - 5 genral purpose
const REG_B :usize = 4;
const REG_C :usize = 4;
const REG_D :usize = 5;
const REG_BP :usize = 6; // base pointer
const REG_FLAGS :usize = 7; // flage register

//flag bits
const FLAG_ZERO :u16 = 1 << 0;
const FLAG_CARRY :u16 = 1 << 1;
const FLAG_SIGN :u16 = 1 << 2;
const FLAG_OVERFLOW :u16 = 1 << 3;

fn main() {
    println!("Hello, world!");
}
