use tokio::time;

// 非同期処理とスクレイピング
// ダミーのmain関数
fn main() {}

/*
Rustで非同期処理を行うには、以下のクレートを使用
- Tokio: 特にこだわりがなければこちらを使うとよい
- async-std
*/

// 非同期処理の基本
// async_test
#[tokio::test] // テストとして実行するためにtokio::mainではなくtokio::testを指定
async fn async_test_main() {
    // 非同期関数を準備
    let f = say_later("諦めるのに時がある。");

    // メッセージを表示
    println!("探すのに時がある。");

    // 非同期処理を実行
    f.await;
}

// 非同期関数を実行
async fn say_later(msg: &'static str) {
    // メッセージを表示
    println!("{}", msg);
}

// 連続で非同期処理を実行子戻り値を得る
// async_longtime
#[tokio::test]
async fn async_longtime_main() {
    // 非同期処理を連続で実行
    for i in 1..=3 {
        println!("#{}を開始", i);
        // 非同期関数を実行して結果を得る
        let s = read_longtime().await;
        println!("{}", s);
        // 非同期処理波ブロックでも記述可能
        let s = async {
            time::sleep(time::Duration::from_secs(1)).await;
            String::from("長い読み込み完了(block)")
        }
        .await;
        println!("{}", s);
    }
}

// 実行に時間がかかる関数
async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("長い読み込み完了(fn)")
}

// 非同期処理で並列処理
// async_spawn
// use tokio::time;

// sec秒後にmsgを表示する非同期関数
async fn say_later_spawn(sec: u64, msg: &str) {
    time::sleep(time::Duration::from_secs(sec)).await;
    println!("{}: {}", sec, msg);
}

#[tokio::test]
async fn async_spawn_main() {
    // spawnで並列実行する
    tokio::spawn(say_later_spawn(3, "毎日が宴会である。"));
    tokio::spawn(say_later_spawn(2, "陽気な人の心には..."));
    tokio::spawn(say_later_spawn(1, "苦しむ人にはどの日も悪い日で..."));
    // 並列実行の完了まで待機
    time::sleep(time::Duration::from_secs(4)).await;
    println!("------");

    // join!で並列実行する
    tokio::join! (
        say_later_spawn(2, "一生懸命働く充実感..."),
        say_later_spawn(3, "人にとってこれ以上の幸せはない。"),
        say_later_spawn(1, "食べ、飲み..."),
    );
}

// webサイトにある画像を連続でダウンロード
// scraping_shodou
use scraper::Selector;
use std::{fs::File, io::Write};
// use tokio::time;

#[tokio::test]
async fn scraping_shodou_main() {
    // 特定タイトルの作品の一覧をダウンロード
    for title in ["温泉", "書道"] {
        download_images(title).await;
    }
}

// 書道サイトから指定のタイトルの画像をダウンロード
async fn download_images(title: &str) {
    let shodou_url = "https://uta.pw/shodou";
    // 指定のタイトルの作品を検索
    let url = format! (
        "{}/index.php?titles&show&title={}",
        shodou_url,
        urlencoding::encode(title));
    // HTMLを取得
    println!("get: {}", url);
    let html = reqwest::get(url).await.unwrap().text().await.unwrap();
    // HTMLをパース
    let doc = scraper::Html::parse_document(&html);
    // imgタグを取り出す
    let selector = Selector::parse(".articles img").unwrap();
    for (i, node) in doc.select(&selector).enumerate() {
        // <img src="***">のsrc属性を取り出す
        let src = node.value().attr("src").unwrap();
        let img_url = format!("{}/{}", shodou_url, src);
        println!("{}", img_url);
        // ダウンロードしてファイルに保存
        let filename = format!("src/ch6/scraping_shodou_download/shodou_{}_{}.png", title, i);
        let bytes = reqwest::get(img_url).await.unwrap().bytes().await.unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(&bytes).unwrap();
        // 待機時間を入れる(重要)
        time::sleep(time::Duration::from_millis(1000)).await;
    }
}

