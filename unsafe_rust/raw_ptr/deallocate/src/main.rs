use std::alloc::{dealloc, Layout};

#[allow(unused_variables, unused_unsafe)]

struct MyStruct {
    num: i32,
}
fn main() {
    let my_struct = MyStruct { num: 5 };
    let struct_boxed = Box::new(my_struct);
    let struct_ptr = &*struct_boxed as *const MyStruct;

    println!("struct_ptr address {:p}", struct_ptr);
    println!("struct_boxed address {:p}", struct_boxed);

    unsafe {
        dealloc(
            struct_ptr as *mut _,
            Layout::for_value(struct_ptr.as_ref().unwrap()),
        )
    };
    println!(
        "struct_ptr derreference should be 0 as its deallocated: {}",
        unsafe { (*struct_ptr).num }
    );

    unsafe { println!("struct_ptr.num {}", struct_ptr.read().num) }
}
