fn main() {
    let n = 1;
    let n_ref: &i32 = &n;
    println!("{}", n);
    println!("{}", n_ref);

    assert_eq!(1, n);
    assert_eq!(1, *n_ref);

    let s = String::from("hello");
    let s_ref: &String = &s;
    let s_str: &str = s_ref;
    println!("{}", s);
    println!("{}", *s_ref);
    println!("{}", s_str);


}