pub fn main() {
    let random_addr_ptr = 0x0000Ausize as *const u8;

    let num = unsafe { *random_addr_ptr };

    println!("Segfault incoming :) {}", num);
}
