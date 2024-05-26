// iterator
// ダミーのmain
fn main() {}

// イテレータの基本を確認
// イテレータとは、配列などの複数要素を持つ集合的データに対して、各要素の繰り返し処理を実現するために用いる抽象表現
// iter_ranges.rs
#[test]
fn iter_ranges_main() {
    // 1~7のうち奇数の値のみ出力する
    for i in 1..=7 {
        if i % 2 == 1 {
            println!("{}", i);
        }
    }
}

// 配列の要素を繰り返す方法
// iter_array.rs
#[test]
fn iter_array_main() {
    // 配列を初期化
    let array = [1, 3, 5, 7];
    // 配列の要素を繰り返す
    for a in array {
        println!("{}", a);
    }
    // 配列の要素数を表示
    println!("len={}", array.len());
}

// 所有権の問題でfor分がうまくいかない例(String型など)
// iter_array_string_err.rs
#[test]
fn iter_array_string_err_main() {
    // String型の配列を初期化
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];
    // 配列の要素を繰り返す
    // for a in array {  // ここで所有権が移動する。
    //     // 配列をfor文に使うと暗黙的にinto_iter()が呼ばれる。
    //     // これにより、所有権が移動してしまうため、arrayを使うことができなくなる。
    for a in array.iter() {
        // iter()で所有権を移動しないようにする
        /*
        iter: 値の参照(&T)のイテレータを返す。所有権を移動しない。
        iter_mut: ミュータブルな値の参照(&mut T)のイテレータを返す。所有権を移動しない。
        into_iter: 値(T)のイテレータを返す。所有権を移動する。配列を直接使うと暗黙的のこれが使われる。
        */
        println!("{}", a);
    }
    // 配列の要素数を表示
    println!("len={}", array.len()); // for文でin arrayとしたとき、(array.into_iter()を使ったとき)、arrayに所有権が無いためエラー
}

// イテレーターはトレイトとして定義されている
//
/*
Iteratorトレイトの実装は以下
pub trait Iterator {
    type Item; <- トレイトの関連型。イテレータが返す要素の型
    fn next(&mut self) -> Option<Self::Item>;
    // (以下略)
}

イテレータを自分で作るときは、
・Item型を定義すること
・nextメソッド(Item型をOptionで包んで返す)を実装すること。次の要素をSome(値)で返すか、伽の値がもうない場合はNoneを返す。
*/

// 素数を求めるイテレータを作る
// iter_prime.rs
// 素数を返す8ビットイテレータ
struct PrimeIterator {
    n: u8,
}
// メソッドを定義
impl PrimeIterator {
    fn new() -> Self {
        PrimeIterator { n: 1 }
    }
    // self.nが素数かどうかを判定する
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        true
    }
}
// イテレータを実装
impl Iterator for PrimeIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            // 8ビットを超える素数を調べない
            if self.n == std::u8::MAX {
                return None;
            }
            // self.nが素数か判定し、素数ならself.nをSomeで包んで返す
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}
// 実行
#[test]
fn iter_prime_main() {
    // 素数イテレータを初期化
    let prime_iter = PrimeIterator::new();
    // 素数を出力
    for n in prime_iter {
        print!("{},", n);
    }
}

// フィボナッチ数列を返すイテレータ
// iter_fib.rs
struct FibIterator {
    a: usize,
    b: usize,
}
impl FibIterator {
    fn new() -> Self {
        FibIterator { a: 1, b: 1 }
    }
}
// イテレータを自走
impl Iterator for FibIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        Some(self.a)
    }
}
#[test]
fn iter_fib_main() {
    // フィボナッチ数列イテレータを初期化
    let fib_iter = FibIterator::new();
    // フィボナッチ数列を出力
    // take(整数)で先頭整数個の値が取得できる
    // for_eachで取得した各値に対して処理を実行する
    fib_iter.take(10).for_each(|f| print!("{},", f));
}

// Iteratorトレイトを実装すると、自動的にenumerateややtakeメソッドなどイテレータで利用できるメソッドが使えるようになる