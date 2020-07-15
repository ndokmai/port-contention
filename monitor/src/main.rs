#![feature(asm)]
use std::env;

const A: f64 = 1.; 
//const A: f64 = f64::NAN;
const B: f64 = 1.;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let n = args[1].parse::<usize>().unwrap(); // # of data points generated
    let mut results = vec![0; n];


    eprintln!("Measuring...");
    for r in results.iter_mut() {
        *r = measure(); 
    }
    eprintln!("monitor done");
    for r in results {
        println!("{}", r);
    }
}

#[inline]
fn measure() -> u64 {
    let mut result: u64;
    unsafe {
        asm!("
        fld qword ptr [{ptr_b}] 
        fld qword ptr [{ptr_a}] 
        rdtscp
        shl rdx, 32 
        or rax, rdx
        mov r8, rax
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        fdivp st, st(1)
        rdtscp
        shl rdx, 32 
        or rax, rdx
        sub rax, r8
        fstp st
        fstp st
        ", 
        ptr_a = in(reg) &A, 
        ptr_b = in(reg) &B, 
        out("rax") result)
    }
    result
}
