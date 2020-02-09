use async_std::task;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    println!("future created!");
    task::block_on(future); // `future` is run and "hello, world!" is printed
}
