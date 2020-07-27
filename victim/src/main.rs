#![feature(asm)]
use std::io::Write;

const FREE_HANDLE: usize = 1234;
//const REPEAT: usize = 0;
const IOCTL_SET_VICTIM_PID: u64 = 2148344065;
const IOCTL_SET_NUKE_ADDR: u64 = 2148344066;
const IOCTL_PREP_PF: u64 = 2148344068;

const DEVICE_PATH: &str = "/home/ndokmai/workspace/MicroScope/nuke_channel";
fn main() {
    let fd = unsafe { libc::open(DEVICE_PATH.as_ptr() as *const libc::c_char, libc::O_RDWR) };
    format!("{}", fd); // DON'T REMOVE I DON'T KNOW WHY BUT THIS IS NEEDED
    if fd == -1 {
        println!("Open Device Error: {}", unsafe { *libc::__errno_location() });
        std::process::exit(1);
    }

    let pid = std::process::id();
    let mut pid_buf = Vec::new();
    write!(&mut pid_buf, "0x{:x}", pid).unwrap();
    
    let result = unsafe { libc::ioctl(fd, IOCTL_SET_VICTIM_PID, pid_buf.as_ptr()) };
    if result == -1 {
        println!("VICTIM_PID Error: {}", unsafe { *libc::__errno_location() });
        std::process::exit(1);
    }

    let mut va_buf = Vec::new();
    write!(&mut va_buf, "{:?}", (&FREE_HANDLE) as *const usize).unwrap();
    
    let result = unsafe { libc::ioctl(fd, IOCTL_SET_NUKE_ADDR, va_buf.as_ptr()) };
    if result == -1 {
        println!("NUKE_ADDR Error: {}", unsafe { *libc::__errno_location() });
        std::process::exit(1);
    }

    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut secrets = [0u8; 30];
    rng.fill_bytes(&mut secrets[..]);
    let secrets = secrets.iter().map(|v| *v & 1).collect::<Vec<u8>>();

    println!("{:?}", secrets);

    unsafe {
        for secret in secrets.iter() {
            let a: f64 = 7.;
            let b: f64 = f64::NAN;
            let c: f64 = 8.;
            let mut _free_handle_var: usize = 0; 

            let result = libc::ioctl(fd, IOCTL_PREP_PF);
            if result == -1 {
                println!("PF Error: {}", *libc::__errno_location());
                std::process::exit(1);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));

            // replay handle
            asm!("mov {free_handle_var}, qword ptr [{free_handle_ptr}]", 
                free_handle_var = in(reg) _free_handle_var, 
                free_handle_ptr = in(reg) &FREE_HANDLE,
            );
            if *secret == 0 {
                    asm!("
                fld qword ptr [{ptr_c}] 
                fld qword ptr [{ptr_a}] 
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
                fstp st 
                ", 
                ptr_c = in(reg) &c, 
                ptr_a = in(reg) &a);
            } else {
                    asm!("
                fld qword ptr [{ptr_c}] 
                fld qword ptr [{ptr_b}] 
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
                fstp st 
                ", 
                ptr_c = in(reg) &c, 
                ptr_b = in(reg) &b);
            }
        }
    }
    eprintln!("victim: done!");
}
