#![feature(asm)]
//#![allow(unused_assignments)]
use std::env;

#[inline]
fn measure() -> u64 {
    let mut result: u64;
    let norm: f64 = 7.;
    //let subnorm: f64 = 1e-310;
    let subnorm: f64 = 1.;
    let output: f64 = 0.;
    unsafe {
        asm!("
        fld qword ptr [r10] 
        fld qword ptr [r9] 
        rdtscp
        shl rdx, 0x20
        or rax, rdx
        mov r11, rax
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        fdiv st(0), st(1)
        rdtscp
        shl rdx, 0x20
        or rax, rdx
        sub rax, r11
        mov {0}, rax
        fst qword ptr [r8]
        ", out(reg) result, in("r8") &output, in("r9") &norm, in("r10") &subnorm);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let n = args[1].parse::<usize>().unwrap(); // # of data points generated
    let mut results = vec![0; n];

    // warmup
    eprintln!("Warming up...");
    for _ in 0..n {
        measure();
    }

    eprintln!("Measuring...");
    for r in results.iter_mut() {
        *r = measure();
    }

    for r in results {
        println!("{}", r);
    }
}
