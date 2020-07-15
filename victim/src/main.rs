#![feature(asm)]
use rand::RngCore;

const A: f64 = 7.;
const B: f64 = f64::NAN;
const C: f64 = 8.;
const REPEAT: usize = 10000;

fn main() {
    let mut rng = rand::thread_rng();
    let mut secrets = [0u8; 30];
    rng.fill_bytes(&mut secrets[..]);
    let secrets = secrets.iter().map(|v| *v & 1).collect::<Vec<u8>>();
    println!("secrets = {:?}", secrets);
    unsafe {
        for secret in secrets.iter() {
            if *secret != 0 {
                for _ in 0..REPEAT {
                    asm!("
                fld qword ptr [{ptr_c}] 
                fld qword ptr [{ptr_a}] 
                fdivp st, st(1)
                fstp st 
                nop
                nop
                nop
                nop
                nop
                nop
                ", 
                ptr_c = in(reg) &C, 
                ptr_a = in(reg) &A);
                }
            } else {
                for _ in 0..REPEAT {
                    asm!("
                fld qword ptr [{ptr_c}] 
                fld qword ptr [{ptr_b}] 
                fdivp st, st(1)
                fstp st 
                nop
                nop
                nop
                nop
                nop
                nop
                ", 
                ptr_c = in(reg) &C, 
                ptr_b = in(reg) &B);
                }
            }

        }
    }
    println!("victim done");
}
