// メッセージングアプリのクライアント側実装
// chat_client.rs
use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // サーバーのアドレスとポート番号を指定
    let server_addr = "127.0.0.1:8888";
    // サーバーと接続
    let mut socket = TcpStream::connect(server_addr).expect("接続に失敗しました");
    socket.set_nonblocking(true).expect("利用不可");
    println!("{}に接続しました", server_addr);
    // 受信用のスレッドを開始
    start_thread_client(socket.try_clone().unwrap());
    // 標準入力からユーザー名を得る
    let user = input("お名前は?");
    println!("{}さん、メッセージを入力してください。", user);
    loop {
        // 標準入力からメッセージを取得してサーバーに送信
        let msg = input("");
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

// クライアントのスレッドを開始してサーバーからメッセージを受診する
fn start_thread_client(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        // サーバーからメッセージを受信
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                // 受診内容をコンソールに表示
                println!("[受信] {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

// 標準入力から文字列を取得
fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("標準入力エラー");
    String::from(buf.trim())
}
