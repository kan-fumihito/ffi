extern crate cmake_example;

fn main() {
    let i:i32;
    unsafe {
        i=cmake_example::foo();
        
    }
    println!("{}",i);
}
