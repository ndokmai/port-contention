#![feature(asm)]
use std::env;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const A: f64 = 1.; 
//const A: f64 = f64::NAN;
const B: f64 = 1.;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let result_filename = &args[1];
    let mut result_file = std::fs::File::create(result_filename).unwrap();
    let mut results = Vec::with_capacity(10000000);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    eprintln!("monitor: measuring...");
    while running.load(Ordering::SeqCst) {
        results.push(measure()); 
    }
    eprintln!("monitor: writing results...");
    for r in results {
        writeln!(result_file, "{}", r).unwrap();
    }
    eprintln!("monitor: done!");
}

#[inline]
fn measure() -> u64 {
    let mut result: u64;
    unsafe {
        asm!("
        fld qword ptr [{ptr_b}] 
        fld qword ptr [{ptr_a}] 
        lfence
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
        fdivp st, st(1)
        fdivp st, st(1)
        lfence
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
