use std::env;
use core::arch::x86_64::_rdtsc;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==2);
    let secret = match args[1].as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid secret value")
    };

    let rounds = 1_000;
    let norm = vec![7.; rounds];
    let subnorm = vec![1e-310; rounds];
    let ones = vec![1.; rounds];
    let mut test_results = vec![0.; rounds];

    if cfg!(feature = "measure") {
        loop {
            unsafe {
                let now = _rdtsc();
                for i in 0..rounds {
                    let one = ones[i];
                    let a = norm[i];
                    let b = subnorm[i];
                    let c = if secret {
                        one*a
                    } else {
                        one*b
                    };
                    test_results[i] = c;
                }
                println!("{}", _rdtsc() - now);
            }
        }
    } else {
        loop {
            for i in 0..rounds {
                let one = ones[i];
                let a = norm[i];
                let b = subnorm[i];
                let c = if secret {
                    one*a
                } else {
                    one*b
                };
                test_results[i] = c;
            }
        }
    }
}
