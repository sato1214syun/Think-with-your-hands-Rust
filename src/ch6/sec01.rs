// マクロの作り方
// ダミーのmain関数
fn main() {}

// マクロを定義する
// macro_echo_num.rs
// 数値を画面に表示するマクロを定義
macro_rules! echo_num {
    // expr(式を意味する構文規則)を$numという変数に置き換える
    ($num:expr) => {
        println!("{}", $num);
    };
}

// マクロを使用する
#[test]
fn macro_echo_num_main() {
    // 引数の指定部分部分のカッコは(), [], {}のどれでもよい
    echo_num!(10);
    echo_num![10];
    echo_num! {10} // {}の後ろの;は省略できる
}

// 可変引数のマクロを定義する方法
// macro_echo_nums.rs
#[macro_export] // このマクロを、このクレートを取り込んだスコープでも使えるようにする
macro_rules! echo_nums {
    // ($(...), *)と書くと、カンマで区切られた複数の引数を受け取ることができる
    // ($(...); *)と書くと、セミコロンで区切られた複数の引数を受け取ることができる
    ($($num:expr), *) => {
        // $(...)*の部分で、複数の引数を一つずつ順番に繰り返し処理する
        $(
            print!("{}, ", $num);
        )*
        println!();
    };
}

// マクロを使用する
#[test]
fn macro_echo_nums_main() {
    // 引数の指定部分部分のカッコは(), [], {}のどれでもよい
    echo_nums![10, 20, 30, 40, 50];
}

// マクロを使ってBASIC風のfor文を実装する(for i = 1 to 10やfor i = 1 to 10 step 2など)
// macro_for.rs

// BASICライクなfor文を実装するマクロ
macro_rules! easy_for {
    // for i = 1 to 10のような場合
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};
    // for i = 1 to 10 step 2のような場合
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }};
    // for i = 10 to 0 step -2のような場合
    (
        for $i:ident = $from:tt to $to:tt step -$step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i < $to { break }
            $block
            $i -= $step
        }
    }};
}

#[test]
fn macro_for_main() {
    // マクロを利用して1から10の合計を求める
    let mut sum = 0;
    easy_for! {
        for i = 1 to 10 {
            sum += i;
        }
    }
    println!("sum = {}", sum);

    // マクロを利用して0から10まで3刻みで表示する
    easy_for! {
        for i = 0 to 10 step 3 {
            println!("i={}", i);
        }
    }
    // マクロを利用して10から0まで-3刻みで表示する
    easy_for! {
        for i = 10 to 0 step -3 {
            println!("i={}", i);
        }
    }
}

// HashMapを手軽に二初期化するマクロ
// macro_hashmap.rs

// HashMapを初期化するマクロ
macro_rules! map_init {
    ( $($key:expr => $val:expr), *) => {{
        // HashMapを生成
        let mut tmp = std::collections::HashMap::new();
        // 値を繰り返し挿入
        $(
            tmp.insert($key, $val);
        )*
        tmp // オブジェクトを返す
    }}
}

#[test]
fn macro_hashmap_main() {
    // マクロを利用してHashMapを初期化
    let week = map_init![
        "mon" => "月曜",
        "tue" => "火曜",
        "wed" => "水曜",
        "thu" => "木曜",
        "fri" => "金曜",
        "sat" => "土曜",
        "sun" => "日曜"
    ];
    println!("mon={}", week["mon"]);
    println!("wed={}", week["wed"]);
}

// 肥満度判定を舞うろで簡略記述
// macro_bmi.rs

macro_rules! bmi_select {
    //パターンを指定
    ( $bmi:expr; $( $label:expr => $range:expr);+) => {{
        // マクロのデフォルト戻り値
        let mut result = "error";
        // 繰り返し
        $ (
            if $range.start <= $bmi && $bmi < $range.end {
                result = $label;
            }
        ) +
        result
    }};
}

#[test]
fn macro_bmi_main() {
    // 身長と体重を設定
    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);
    // BMIを判定するマクロを実行
    let label = bmi_select! [
        bmi;
        "低体重" => 0.0..18.5;
        "普通" => 18.5..25.0;
        "肥満1度" => 25.0..30.0;
        "肥満2度" => 30.0..35.0;
        "肥満3度" => 35.0..40.0;
        "肥満4度" => 40.0..99.0
    ];
    println!("label={}", label);
}

// マクロの再帰呼び出し
// macro_html.rs

// htmlを出力する
macro_rules! output_html {
    // 引数なしの場合、空を返す
    () => {()};
    // 引数が1つだけの場合
    ($e:tt) => {print!("{}", $e)};
    // タグ[内側]続きを指定した場合
    ($tag:ident [$($inner:tt)*] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        output_html!($($inner)*);
        println!("</{}>", stringify!($tag));
        output_html!($($rest)*);
    }};
}

#[test]
fn macro_html_main() {
    // マクロを利用してHTML構造を手軽に出力
    output_html! (
        html[
            head[title["test"]]
            body[
                h1["test"]
                p["This is test."]
            ]
        ]
    );
}
