// 配列をシャッフルするのに必要な宣言 --- (*1)
use rand::seq::SliceRandom;

fn main() {
    // 1から75までの数値を配列に代入 --- (*2)
    let mut numbers = vec![];
    for i in 1..=75 {
        numbers.push(i);
    }

    // シャッフル --- (*3)
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);

    // カードを表示 --- (*4)
    let row = 5;
    let col = 5;
    let center = 12;
    for i in 0..row * col {
        if i % col == 0 {
            println!("");
        }
        if i == center {
            // ワイルドカードの時
            print!("  *,");
        } else {
            // 通常
            print!("{:3},", numbers[i]);
        }
    }
    println!("")
}
