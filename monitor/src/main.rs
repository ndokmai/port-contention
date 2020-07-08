use core::arch::x86_64::_rdtsc;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==3);
    let n = args[1].parse::<usize>().unwrap(); // # of data points generated
    let rounds = args[2].parse::<usize>().unwrap(); // # of rounds to run a single instruction
    let subnorm = vec![1e-310; rounds];
    let ones = vec![1.; rounds];
    let mut test_results = vec![0.; rounds];

    // warmup
    eprintln!("Warming up...");
    for _ in 0..n {
        for i in 0..rounds/2 {
            let one = ones[i];
            let a = subnorm[i];
            let b = one*a;
            test_results[i] = b;
        }
    }

    eprintln!("Measuring...");
    for _ in 0..n {
        let mut s = 0;
        for i in 0..rounds {
            let one = ones[i];
            let a = subnorm[i];
            unsafe {
                let now = _rdtsc(); 
                let b = one*a;
                s += _rdtsc() - now;
                test_results[i] = b;
            }
        }
        println!("{}", s);
    }
}
