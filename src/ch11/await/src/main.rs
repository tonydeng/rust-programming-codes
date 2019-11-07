#[cfg(test)]

use futures::executor::block_on;

mod first{

use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world();
    block_on(future);
}
#[test]
fn run_main(){
    main()
}
}

#[derive(Debug)]
struct Song;

async fn learn_song() -> Song {
    println!("learn_song {:?} ...",Song);
    Song
}
async fn sing_song(_: Song) {
    println!("sing_song {:?} ...",Song);
}
async fn dance(){
    println!("dance...")
}

mod second {

use super::*;

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}

#[test]
fn run_second_main() {
    main();
}

}


mod third{
    use super::*;

    async fn learn_and_sing() {
        let song = learn_song().await;
        sing_song(song).await;
    }

    async fn async_main(){
        let f1 = learn_and_sing();
        let f2 = dance();

        futures::join!(f1, f2);
    }

    fn main() {
        block_on(async_main());
    }

    #[test]
    fn run_main() {
        main();
    }
}
// async fn hello_world() {
//     println!("Hello, world!");
// }

// #[derive(Debug)]
// struct Song {
    
// }

// async fn learn_song() -> Song { 
//     Song{}
// }
// async fn sing_song(song: Song) {
//     Song{}
// }
// async fn dance() {

// }

// fn main() {


//    let future = hello_world();
//    block_on(future);
// }
