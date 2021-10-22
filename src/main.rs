extern crate rclwasm;

fn main() {
    unsafe {
        let i = rclwasm::foo();
        println!("return {} from rclwasm::foo.",i);
        rclwasm::bar();
    }
}
