use tokio::time;



fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        time::sleep(std::time::Duration::from_secs(5)).await;
        println!("hello");
    });

    println!("main");

}