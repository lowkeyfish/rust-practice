fn main() {
    let s = String::from("s");
    let s_box = Box::new(&s);

    println!("{s}");
    println!("{s_box}");
    
    println!("{:?}", s.as_ptr());
    println!("{:?}", s_box.as_ptr());

    let n = 1;
    let n_box = Box::new(&n);
    println!("{:?}", &n);
    println!("{:?}", &n_box);
}