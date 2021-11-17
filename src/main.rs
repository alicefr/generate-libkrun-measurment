use libc::{c_char, size_t};

#[link(name = "krunfw")]
extern "C" {
    fn krunfw_get_qboot(size: *mut size_t) -> *mut c_char;
    fn krunfw_get_initrd(size: *mut size_t) -> *mut c_char;
    fn krunfw_get_kernel(load_addr: *mut u64, size: *mut size_t) -> *mut c_char;
}

fn print_libkrun_measurment() {
    let mut kernel_guest_addr: u64 = 0;
    let mut kernel_size: usize = 0;
    let kernel_host_addr = unsafe {
        krunfw_get_kernel(
            &mut kernel_guest_addr as *mut u64,
            &mut kernel_size as *mut usize,
        )
    };

    let mut qboot_size: usize = 0;
    let qboot_host_addr = unsafe { krunfw_get_qboot(&mut qboot_size as *mut usize) };

    let mut initrd_size: usize = 0;
    let initrd_host_addr = unsafe { krunfw_get_initrd(&mut initrd_size as *mut usize) };

    let qboot_data =
        unsafe { std::slice::from_raw_parts(qboot_host_addr as *const u8, qboot_size) };
    let kernel_data =
        unsafe { std::slice::from_raw_parts(kernel_host_addr as *const u8, kernel_size) };
    let initrd_data =
        unsafe { std::slice::from_raw_parts(initrd_host_addr as *const u8, initrd_size) };
    println!("qboot_size {:?}", qboot_size);
    println!("kernel_size {:?}", kernel_size);
    println!("initrd_size {:?}", initrd_size);
}

fn main() {
    print_libkrun_measurment();
}
