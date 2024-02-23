
fn main() {
    trait Test {
        fn test() {
            println!("test");
        }
    }

    struct Test1 {
        a: i32
    }

    impl Test1 {
        fn test1(&self) {
            println!("{}", self.a);
        }
    }

    impl Test for Test1 {
        
    }

    let t = Test1 { a: 1 };
    Test1::test();
    t.test1();
}