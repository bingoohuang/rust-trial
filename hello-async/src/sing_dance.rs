use async_std::task::block_on;
use futures::join;
use std::{thread, time};

#[derive(Debug)]
struct Song<'a> {
    name: &'a str,
}

async fn learn_song() -> Song<'static> {
    Song {
        name: "bingoohuang",
    }
}

async fn sing_song(song: Song<'static>) {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    println!("sing song {:?}", song)
}

async fn dance() {
    println!("have a dance");
}

async fn learn_and_sing() {
    // 在唱歌之前等待学歌完成
    // 这里我们使用 `.await` 而不是 `block_on` 来防止阻塞线程，这样就可以同时执行 `dance` 了。
    let song = learn_song().await;
    sing_song(song).await;
}
async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // `join!` 类似于 `.await` ，但是可以等待多个 future 并发完成
    // 如果学歌的时候有了短暂的阻塞，跳舞将会接管当前的线程，如果跳舞变成了阻塞
    // 学歌将会返回来接管线程。如果两个futures都是阻塞的，
    // 这个‘async_main'函数就会变成阻塞状态，并生成一个执行器
    join!(f1, f2);
}
fn main() {
    block_on(async_main());
}
