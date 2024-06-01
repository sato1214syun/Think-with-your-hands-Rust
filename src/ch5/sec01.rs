// 画像処理ツールを作ろう
//ダミーのmain関数
fn main() {}

// 市松模様を描画するプログラム
// image_ichimatu/src/main.rs
use image;
#[test]
fn image_ichimatu_main() {
    // 白色をRGBで定義
    let white = image::Rgb::<u8>([255, 255, 255]);
    // 赤色をRGBで定義
    let red = image::Rgb::<u8>([255, 90, 90]);
    // 1マスのサイズ
    let w = 64;
    // 市松模様を描くクロージャ
    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);
        match (xi % 2) ^ (yi % 2) {
            0 => white,
            1 => red,
            _ => panic!("error"),
        }
    };
    // クロージャを指定してImageBufferを生成
    let img = image::ImageBuffer::from_fn(512, 512, draw);
    // ファイルへ保存
    img.save("./src/ch5/sec01_result/ichimatu.png").unwrap();
}

// 画像の正方形サムネイルを作成
// image_thumb
use image::{imageops, GenericImageView};
#[test]
fn image_thumb_main() {
    // リサイズするサイズ
    let size = 128;
    // コマンドライン引数を取得
    // let args: Vec<String> = std::env::args().skip(1).collect();
    let args = Vec::from(["./sample-src/ch5/image_thumb/photo.jpg".to_string()]);
    if args.len() < 1 {
        println!("[USAGE] image_thumb imagefile");
        return;
    }
    // 入力ファイルと出力ファイルを指定
    let input_file = String::from(&args[0]);
    let output_file = format!("./src/ch5/sec01_result/{}-thumb.png", input_file.split("/").last().unwrap());
    println!("input: {}", input_file);
    println!("output: {}", output_file);
    // 画像ファイルを読み込む
    let mut img = image::open(input_file).expect("画像ファイルが読み込めません");
    // 画像ファイルを得る
    let dim = img.dimensions();
    // 正方形に切り取る
    let w = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img2 = imageops::crop(&mut img, (dim.0 - w) / 2, (dim.1 - w) / 2, w, w).to_image();
    // 指定サイズにリサイズ
    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
    // ファイルへ保存
    img3.save(output_file).unwrap();
}


// ネガポジ反転
// image_filter
use image::{GenericImage, Rgba};
#[test]
fn image_filter_main() {
    // コマンドライン引数を取得
    // let args: Vec<String> = std::env::args().skip(1).collect();
    let args = Vec::from(["./sample-src/ch5/image_filter/photo.jpg".to_string()]);
    if args.len() < 1 {
        println!("[USAGE] image_filter imagefile");
        return;
    }
    // 入力ファイルと出力ファイルを指定
    let input_file = String::from(&args[0]);
    let output_file = format!("./src/ch5/sec01_result/{}-filter.png", input_file.split("/").last().unwrap());
    println!("input: {}", input_file);
    println!("output: {}", output_file);
    // 画像ファイルを読み込む
    let mut img = image::open(input_file).expect("画像ファイルが読み込めません");
    // 画像の幅と高さを取得
    let (w, h) = img.dimensions();
    // 行と列を走査してピクセルを取得
    for y in 0..h {
        for x in 0..w {
            // ピクセルの色を取得
            let c = img.get_pixel(x, y);
            // ネガポジ反転
            let c = Rgba([
                255 - c[0],  // 赤
                255 - c[1],  // 緑
                255 - c[2],  // 青
                c[3],  // 透明度
            ]);
            // 反転したピクセルを書き込む
            img.put_pixel(x, y, c);
        }
    }
    // 画像を保存
    img.save(output_file).unwrap();
}