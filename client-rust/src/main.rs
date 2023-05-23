#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;

include!("../bindings.rs");

fn main() {
    println!("Using awesome lib from Rust:\n");

    //Call Add() - passing integer params, interger result
    let a: GoInt = 12;
    let b: GoInt = 99;
    println!("awesome.Add(12,99) = {}\n", unsafe { Add(a, b) });

    //Call Cosine() - passing float param, float returned
    println!("awesome.Cosine(1) = {}\n", unsafe { Cosine(1.0) });

    //Call Sort() - passing an array pointer
    let mut data: [GoInt; 6] = [77, 12, 5, 99, 28, 23];
    let nums = GoSlice {
        data: data.as_mut_ptr() as *mut c_void,
        len: 6,
        cap: 6,
    };
    unsafe { Sort(nums) };
    println!("awesome.Sort(77,12,5,99,28,23): ");
    for i in 0..6 {
        println!("{},", nums.data.wrapping_offset(i as isize) as GoInt);
    }
    println!("\n");

    //Call Log() - passing string value
    let msg = GoString {
        p: "Hello from C!".as_ptr() as *mut i8,
        n: 13,
    };
    unsafe { Log(msg) };
}
