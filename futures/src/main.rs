use std::future::Future;
use futures::executor;

// 创建异步函数语法
async fn my_function() {
    println!("Hello Async")
}

// 上面函数相当于这样写
fn my_function2() -> impl Future<Output = ()> {
    async{
        println!("Hello Async2")
    }
}

// ------------------------------------
async fn learn_song(){
    println!("Learn Song");
}

async fn learn_sing() {
    println!("Learn Sing");
}

async fn learn_dance(){
    println!("Learn Dance");
}

async fn learn_sing_adn_song() {
    learn_sing().await;
    learn_song().await;
}

async fn async_man() {
    let f1 = learn_sing_adn_song();
    let f2 =learn_dance();
    futures::join!(f1,f2);
}

fn main() {
    executor::block_on(async_man());
    println!("Hello Word!");
}

/*
a、在 learn_and_sing_song () 中，会先执行 learn_song ()，然后再执行 sing_song ()，两者按照顺序执行；
b、通过 join，能等待多个 Future 完成；
c、当 await 发生阻塞时，不会阻塞当前线程，可以让其它的任务执行（在此例子中，如果在 learn_song 阻塞，则 learn_and_sing_song 会让出当前线程，可以让 dance 执行）。
 */
