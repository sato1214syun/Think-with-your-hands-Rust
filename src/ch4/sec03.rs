// generics
// ダミーのmain
fn main() {}

// ジェネリクスの復習
// vec_i32_char.rs
// Vecではi32でもcharでも同じメソッドで操作が可能。これはジェネリクスを使っているため
#[test]
fn vec_i32_char_main() {
    // i32型のVecを作成
    let mut v1: Vec<i32> = Vec::<i32>::new();
    // 要素を末尾に追加
    v1.push(10);
    v1.push(20);
    v1.push(30);
    // 末尾の要素を取り出す(ベクターからはその要素は削除される)
    v1.pop();

    // 要素を繰り返す
    for i in v1.iter() {
        println!("{}", i);
    }

    // char型のVecを作成
    let mut v2: Vec<char> = Vec::<char>::new();
    // 要素を末尾に追加
    v2.push('a');
    v2.push('b');
    v2.push('c');
    // 末尾の要素を取り出す(ベクターからはその要素は削除される)
    v2.pop();

    // 要素を繰り返す
    for i in v2.iter() {
        println!("{}", i);
    }
}

// ジェネリクス関数を定義する
// ジェネリクスを使うと、関数やメソッドの使い方が共通化できるだけでなく、重複する関数宣言を共通化できる
// add_i32.rs
// add_i32関数を定義
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
// add_f32関数を定義
fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}
// 上記の関数を使ってみる
#[test]
fn add_i_f32_main() {
    println!("{}", add_i32(10, 25));
    println!("{}", add_f32(10.0, 25.0));
}

// add_i32とadd_f32は同じ処理をしているので、ジェネリクスを使って共通化する
// add_generics.rs
// ジェネリクスを使ってadd関数を定義

// fn 関数名<T: トレイト> (引数1: T, 引数2: T, ...) -> 戻り値の型 {}
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    // <>内でジェネリクスTを定義している。トレイトはstd::ops::Addを指定している
    // Tでなくてもよいが、慣例的にTを使うことが多い。(キャメルケースでないと警告が出る)
    // std::opsは演算子のトレイトを定義したモジュール。Addなので、+演算子を使える型を指定している
    a + b
}
// 上記の関数を実行
#[test]
fn add_generics_main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25)); // ジェネリクス<T>の型を明示することもできる
                                        // println!("{}", add('a', 'a'));  // char型は+演算子が使えないのでエラーになる
}

// トレイト境界
// ジェネリクスの型に対して、上記のstd::ops::Addのようにトレイトを指定することをトレイト境界(trait bound)という。
// トレイトによる使える型の制約を指定子ている

// 値を2倍にするジェネリクス関数を定義する
// x2_generics.rs
fn x2<T: std::ops::Add<Output = T> + Copy>(n: T) -> T {
    // + で複数のトレイトを指定できる
    // Copyにより、同じ変数を繰り返し使えるようになる
    n + n
}
#[test]
fn x2_generics_main() {
    println!("{}", x2(3));
    println!("{}", x2(3.0f64));
    println!("{}", x2::<u64>(3));
}

// ジェネリクス関数でwhereを使う
// add_generics_where.rs
fn add2<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>, // このようにwhereを使ってトレイト境界を設定できる。長いトレイト境界の場合はこちらの方が見やすい
{
    a + b
}
#[test]
fn add_generics_where_main() {
    println!("{}", add2(10, 25));
    println!("{}", add2(10.0, 25.0));
}

// 構造体でジェネリクスを指定する
// struct_generics.rs
#[derive(Debug)] // 構造体の値をprintln!("{:?}")で表示できるようにする
struct Point<T> {
    x: T,
    y: T,
}
#[test]
fn struct_generics_main() {
    let pt_i = Point { x: 10, y: 50 };
    let pt_f = Point { x: 20.5, y: 15.3 };
    println!("{:?}", pt_i);
    println!("{:?}", pt_f);
}

// ジェネリクスを使って構造体にメソッドを実装する
// struct_method_generics.rs
// 構造体Pointを定義
#[derive(Debug)]
struct Point2<T> {
    x: T,
    y: T,
}
// メソッドを定義
impl<T> Point2<T>
where
    T: std::ops::AddAssign,
    // AddAssignトレイトは+=演算子を使えるようにする
{
    // コンストラクター
    fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
    // 加算
    fn add(&mut self, pt: Point2<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}
#[test]
fn struct_method_generics_main() {
    // Point2を生成
    let mut pt1 = Point2::new(10, 10);
    let pt2 = Point2::new(20, 30);
    println!("{:?}", pt1);
    println!("{:?}", pt2);
    // 座標に値を加算
    pt1.add(pt2);
    println!("pt1とp2を加算: {:?}", pt1);
}
