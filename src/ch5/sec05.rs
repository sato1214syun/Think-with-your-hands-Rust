// ネットワークと並列処理
//ダミーのmain関数
fn main() {}

// スレッド
use std::{thread, time};

// 1秒ごとにメッセージを3回表示するスレッド
fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
#[test]
fn threadtest_main() {
    // スレッドなしの場合
    println!("--- スレッドなし ---");
    sleep_print("スレッドなし");

    // スレッドありの場合
    println!("--- スレッドあり ---");
    // スレッド1
    thread::spawn(|| sleep_print("次郎"));
    // スレッド2
    thread::spawn(|| sleep_print("三郎"));
    // メインスレッド
    sleep_print("太郎");
}

// スレッド巻で安全にデータを共有
// mpsctest

// 1秒毎にメッセージを送信する関数
use std::sync::mpsc;
use std::time::Duration;
fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for i in 1..=5 {
        let msg = format!("{}: {}", name, i);
        sender.send(msg).unwrap(); // 送信
        thread::sleep(Duration::from_millis(1000));
    }
    sender.send("quit".to_string()).unwrap();
}

#[test]
fn mpsctest_main() {
    // スレッド間通信のチャンネルを用意
    // tx はmpscのSender, rxはreceiver
    let (tx, rx) = mpsc::channel::<String>();

    // スレッド1を生成
    let sender = tx.clone();
    thread::spawn(|| sleep_sender("太郎", sender));
    // スレッド2を生成
    let sender = tx.clone();
    thread::spawn(|| sleep_sender("次郎", sender));
    // スレッドからのメッセージを繰り返し受ける
    loop {
        let buf = rx.recv().unwrap();
        println!("[受信] {}", buf);
        // quitを受け取ったら終了
        if buf == "quit" {
            break;
        }
    }
}

// スレッドで並列計算処理(フィボナッチ)
// calc_single.rs
use std::time::Instant;

#[test]
fn calc_single_main() {
    // 求めたいf義母ナッチ数の一覧
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    // スレッドなしで計算
    for num in request_nums {
        let answer = fib(num);
        println!("[結果] fib({}) = {}", num, answer);
    }
    show_time(start_time);
}

// 再帰的にフィボナッチ数を調べる関数
fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}
fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("実行時間: {:?}", elapsed);
}

// 並列計算
// calc_multi.rs
#[test]
fn calc_multi_main() {
    // 求めたいf義母ナッチ数の一覧
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    // スレッド間通信のチャンネルを生成
    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num, answer)).unwrap();
        });
    }
    // 生成したスレッドの数を数える
    let mut job = request_nums.len();
    // 計算結果を得る
    loop {
        if let Ok((arg, answer)) = rx.recv() {
            job -= 1;
            println!("[結果]fib({}) = {} (残り={})", arg, answer, job);
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
        thread::sleep(Duration::from_millis(300));
    }
}
