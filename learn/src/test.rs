pub mod test1;
pub mod test2;

pub use test1::show as test1_show;

pub fn show() {
    println!("test");
    test1::show();
    test2::show();
    test1_show();
}