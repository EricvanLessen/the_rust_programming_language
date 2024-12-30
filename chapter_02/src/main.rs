#![allow(dead_code)]
#![allow(unused_imports)]

mod stack_heap;
use std::mem;

fn main() {
    //data_types();
    //operators();
    stack_heap::stack_and_heap();
}

fn data_types(){
    println!("test!");

    let mut a: u8 = 123; // u unsigned
    println!("a = {}", a);

    // a = 456; wont fit in the u8 range
    a = 234;
    println!("a = {}", a);

    let mut b: i8 = -128; // i signed
    println!("b = {}", b);

    b = -64;
    println!("b = {}", b);

    // we have u8, u16, u32, u64, i8, i16, ...
    let mut c = 123456789; // rust figures out the type, here u32

    println!("c = {}", c);
    println!("c takes up {} bytes", mem::size_of_val(&c));
    // 4 bytes, so 32 bits
    c = -1;
    println!("c = {}", c);

    // usize, isize info about the size of types
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",
        z, size_of_z, size_of_z*8);

    let d: char = 'x'; // . ;
    println!("d = {}", d);
    println!("d takes up {} bytes", mem::size_of_val(&d));

    // f32 f64 IEEE754 signed, positive or negative

    let e: f32 = 2.5;
    println!("e takes up {} bytes", mem::size_of_val(&e));

    let f: f64 = 2.5; // default type for non whole numbers
    println!("f takes up {} bytes", mem::size_of_val(&f));

    let g: bool = true;
    println!("g takes up {} bytes", mem::size_of_val(&g));
}

fn operators(){
    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("a_cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}", b, b_cubed);
    println!("{}^pi = {}", b, b_to_pi);

    let c = 1 | 2;
    println!("1 | 2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("Pi is less than 4 is {}", pi_less_4);
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("{} == 5 is {}", x, x_is_5);
}