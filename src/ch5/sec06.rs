// ネットワークと並列処理
//ダミーのmain関数
// fn main() {}

use actix_web::http::header::SERVER;
// actix webでhello world
use actix_web::{web, App, HttpRequest, HttpServer};

// アドレスとポートの指定
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Webのメイン関数
#[actix_web::main]
#[test]
// acyncを頭につけて非同期関数にする
async fn web_hello_main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // HTTPサーバーを起動
    HttpServer::new(|| {
        // ルーティングを指定
        App::new().route("/", web::get().to(index)) // ルート"/"へのアクセスに対して、コールバック関数indexを呼び出す
    })
    // bindでサーバーのアドレスとポートを指定
    .bind(SERVER_ADDR)? // ?は演算子。Resultの場合、Okなら中身を返し、Errならそのまま返す
    .run() // 起動
    .await // 非同期処理を実行
}

// "/"へのアクセス時に実行されるコールバック関数
// acyncを頭につけて非同期関数にする
async fn index(req: HttpRequest) -> &'static str {
    println!("REQUEST: {:?}", req);
    "Hello, Actix Web!"
}

// Actx webでBMI判定を実装
// web_bmi.rs
// use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Error};
use actix_web::{Error, HttpResponse};
use serde::{Deserialize, Serialize};

// アドレスとポートの指定
// const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix webのメイン関数
#[actix_web::main]
#[test]
async fn web_bmi_main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // HTTPサーバーを起動
    HttpServer::new(|| {
        // ルーティングを指定
        App::new()
            .route("/", web::get().to(index_web_bmi))
            .route("/calc", web::get().to(calc))
    })
    // bindでサーバーのアドレスとポートを指定
    .bind(SERVER_ADDR)?
    .run()
    .await
}

// "/"二アクセス子たときのコールバック関数
async fn index_web_bmi(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok() // 正常にアクセスできた場合にどのようなレスポンスを返すか設定
            .content_type("text/html; charset=utf-8")
            .body(format!(
                "{}{}{}{}{}{}",
                "<html><body><h1>BMI判定</h1>",
                "<form action='calc'>", // submitボタンクリック時に飛ぶページ
                "身長: <input name='height' value='160'><br>", // 身長の入力フォームと初期値
                "体重: <input name='weight' value='70'><br>", // 体重の入力フォームと初期値
                "<input type='submit' value='送信'>",
                "</form></body></html>",
            )),
    )
}

// 入力フォームから得られるデータの構造体を定義する
// 入力フォームのname属性と同じ名前でフィールドを定義
// serde_deriveを使ってSerializeとDeserializeを実装しておき、
// 更にコールバック関数の引数にweb::Query<FromBMI>を指定することで、
// 自動的にフォームの値をFromBMI構造体に変換する
#[derive(Serialize, Deserialize, Debug)]
pub struct FromBMI {
    height: f64,
    weight: f64,
}

// "/calc"にアクセスしたときのコールバック関数
// web::Query型を指定することで、自動的にフォームの値をFormBMI構造体に変換する
async fn calc(q: web::Query<FromBMI>) -> Result<HttpResponse, Error> {
    // フォームからパラメータを取れたか確認
    println!("{:?}", q);
    // BMIを計算
    let h = q.height / 100.0;
    let w = q.weight;
    let bmi = w / h.powf(2.0);
    let per = (bmi / 22.0) * 100.0;
    // 結果を表示
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h3>BMI={:.1}, 乖離率={:.0}%</h3>", bmi, per)))
}

// 素早く開発できるWebフレームワーク「Tide」
// web_hello_tide
// const SERVER_ADDR: &str = "127.0.0.1:8888";

// Tideのメイン関数
// #[aync_std::main]で属性を指定するが、関数名mainしか受け付けない
// 次のTideを利用て挨拶するプログラムでも同様なので、干渉子ないようどちらかはコメントアウトしておく

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     println!("http://{}/", SERVER_ADDR);
//     // Tideのオブジェクトを生成
//     let mut app = tide::new();
//     // ルーティングを指定
//     app.at("/").get(|_| async { Ok("Hello, Tide!") });
//     // サーバーを起動
//     app.listen(SERVER_ADDR).await?;
//     Ok(())
// }

// Tideを利用して挨拶するプログラム
// web_iput_name

use tide::prelude::*;

// const SERVER_ADDR: &str = "127.0.0.1:8888";
#[derive(Deserialize, Serialize)]
struct UserInfo {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    // Tideのオブジェクトを生成
    let mut app = tide::new();
    // ルーティングを指定
    app.at("/").get(|_| async {
        // ルートにアクセスしたとき
        // HTMLを出力
        Ok(tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .body(format!(
                "{}{}{}{}",
                "<html><body><form action='hello'>",
                "name: <input name='name' value='kujira'>",
                "<input type='submit' value='送信'>",
                "</form></body></html>",
            ))
            .build())
    });
    // "/hello"にアクセスしたときの処理
    app.at("/hello").get(|req: tide::Request<()>| async move {
        // クエリを解析して構造体に当てはめる
        let user: UserInfo = req.query()?;
        // HTMLを出力
        Ok(tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .body(format!("<h1>Hello, {}!</h1>", user.name))
            .build())
    });
    // サーバーを起動
    app.listen(SERVER_ADDR).await?;
    Ok(())
}
