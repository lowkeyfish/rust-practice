fn main() {
    let mut r = 0;
    let n = loop {
        if r < 10 {
            r += 2;
        } else {
            break r;
        }
    };
    println!("{n}");
}