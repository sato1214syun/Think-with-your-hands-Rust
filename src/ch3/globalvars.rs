// global変数を定義
static mut TAX: f32 = 0.1;

#[test]
fn global_vars() {
    // ミュータブルなstatic変数を扱うためにunsafeを宣言
    unsafe {
        println!("Price: {}", TAX * 300.0);
        TAX = 0.08;
        println!("Price: {}", TAX * 300.0);
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
// グローバル変数として乱数のシードを指定
static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    // 必要ならSEEDを初期化
    if SEED == 0 {
        // 現在時刻を利用して乱数を初期化
        let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }

    // SEEDを利用して乱数を生成
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

#[test]
fn rand_global_main() {
    // 100個の乱数を表示
    let n = 100;
    for _ in 0..n {
        unsafe {
            //乱数を生成して表示
            let v = rand_global(1, 6);
            println!("{}", v)
        }
    }
}

// メモ
// 関数の引数で
// "&変数"を使う場合、所有権を関数に渡さない(参照)
// "&mut 変数"を使う場合、その変数は関数内で値を変更でき、更に関数の呼び出し元でも変数に変更が反映される

// unsafeを使わない疑似乱数生成
// seedをrand関数の引数に入れ、そのseedをrand関数内で変更して乱数を生成する
#[test]
fn rand_xorshift() {
    // 乱数を初期化
    let mut seed = rand_init();
    // 100個の乱数を表示
    let n = 100;
    for _ in 0..n {
        // 乱数を生成
        // &mutで変更可能な参照を指定
        let v = rand(&mut seed, 1, 6);
        println!("{}", v);
    }
}
// // &mutを使わないで力技で実装。seedをrand関数の戻り値で更新している
// fn rand_xorshift() {
//     // 乱数を初期化
//     let mut _seed = rand_init();
//     let mut _v = 0;
//     // 100個の乱数を表示
//     let n = 100;
//     for _ in 0..n {
//         // 乱数を生成
//         (_seed, _v) = rand(_seed, 1, 6);
//         println!("{}", _v);
//     }
// }

fn rand_init() -> u32 {
    // 現在時刻を利用して乱数を初期化
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32
}

fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    // 乱数を生成
    // &mut seed(参照)から値を得るためには*seedを使う
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed % (end - start + 1) + start;
}
// // &mutを使わないで力技で実装
// fn rand(mut seed: u32, start: u32, end: u32) -> (u32, u32) {
//     // 乱数を生成
//     seed ^= seed << 13;
//     seed ^= seed >> 17;
//     seed ^= seed << 5;
//     return (seed, seed % (end - start + 1) + start);
// }
