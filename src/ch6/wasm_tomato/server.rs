use std::io::Result;
use warp;

#[tokio::main]
async fn main() -> Result<()> {
    // ローカルサーバーを起動
    let routes = warp::fs::dir(".");
    warp::serve(routes).run(([127, 0, 0, 1], 8888)).await;

    Ok(())
}
