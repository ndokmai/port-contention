use core::arch::x86_64::_rdtsc;
use ftfp::Fixed;

macro_rules! do_test {
    ($name:expr, $a:expr, $b:expr, $rounds:expr, $op:tt, $s:ident, $test_results:ident) => ({
        $s = 0;
        let vec_a = vec![$a; $rounds];
        let vec_b = vec![$b; $rounds];
        for i in 0..$rounds {
            unsafe {
                let a = vec_a[i];
                let b = vec_b[i];
                let now = _rdtsc();
                let c = a $op b;
                $s += _rdtsc() - now ;
                $test_results[i] = c;
            }
        }
        println!("{} {:.3} {:.3}: \t\t{}", $name, $a, $b, $s);
    })
}

macro_rules! all_tests {
    ($name:expr, $op:tt, $rounds:expr, $s:ident, $test_results:ident) => (
        do_test!(format!("integer {}", $name), 1f64, 0f64, $rounds, $op, $s, $test_results);
        do_test!(format!("subnorm {}", $name), 1e-310f64, 0.1f64, $rounds, $op, $s, $test_results);
        do_test!(format!("subnorm {}", $name), 7f64, 1e-310f64, $rounds, $op, $s, $test_results);
        do_test!(format!("small {}", $name), 0.1f64, 0.1, $rounds, $op, $s, $test_results);
    )
}

macro_rules! all_ftfp_tests {
    ($name:expr, $op:tt, $rounds:expr, $s:ident, $test_results:ident) => (
        do_test!(format!("ftfp-integer {}", $name), Fixed::from(1f64), Fixed::from(0f64), $rounds, $op, $s, $test_results);
        do_test!(format!("ftfp-subnorm {}", $name), Fixed::from(1e-310f64), Fixed::from(0.1f64), $rounds, $op, $s, $test_results);
        do_test!(format!("ftfp-subnorm {}", $name), Fixed::from(7f64), Fixed::from(1e-310f64), $rounds, $op, $s, $test_results);
        do_test!(format!("ftfp-small {}", $name), Fixed::from(0.1f64), Fixed::from(0.1), $rounds, $op, $s, $test_results);
    )
}

#[allow(unused_assignments)]
fn warmup() {
    let rounds = 100000;
    let mut s = 0u64;
    let mut test_results = vec![0f64; rounds];
    do_test!("warmup +", 1f64, 1f64, rounds, +, s, test_results);
    do_test!("warmup *", 1f64, 1f64, rounds, *, s, test_results);
    do_test!("warmup /", 1f64, 1f64, rounds, /, s, test_results);
}

#[allow(unused_assignments)]
fn main() {
    let rounds = 100000;
    let mut s = 0u64;
    println!("# rounds: {}", rounds);
    warmup();
    println!();

    {
        let mut test_results = vec![0f64; rounds];
        all_tests!("+", +, rounds, s, test_results);
        println!();

        all_tests!("*", *, rounds, s, test_results);
        println!();

        all_tests!("/", /, rounds, s, test_results);
        println!();
    }

    {
        let mut test_results = vec![Fixed::default(); rounds];
        all_ftfp_tests!("+", +, rounds, s, test_results);
        println!();

        all_ftfp_tests!("*", *, rounds, s, test_results);
        println!();

        all_ftfp_tests!("/", /, rounds, s, test_results);
    }

}
