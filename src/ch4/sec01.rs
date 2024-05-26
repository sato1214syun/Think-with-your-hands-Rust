// method
// ダミーのmain
fn main() {}

// 構造体にメソッドを定義する方法を学ぶ

// method_bmi
// 身長と体重を表すBody構造体
struct Body {
    height: f64, // 身長cm
    weight: f64, // 体重kg
}
// Body構造体のメソッドを定義
impl Body {
    // BMIを計算するメソッド
    fn calc_bmi(&self) -> f64 {
        // BMI = 体重kg / (身長m * 身長m)
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

// Body構造体を使ってみる
#[test]
fn method_bmi_main() {
    let taro = Body {
        height: 160.0,
        weight: 70.0,
    };
    println!("Taro's BMI: {:.2}", taro.calc_bmi());
    println!("Taro's 乖離率: {:.1}%", taro.calc_per());
}

// struct_new
// Person構造体
struct Person {
    name: String,
    age: i32,
}

// Personのメソッドを定義
impl Person {
    fn new(name: String, age: i32) -> Self {
        // フィールド名と変数が一致しているため省略している(フィールド初期化法)
        Person { name, age } // {name: name, age: age}と同じ
    }
}

#[test]
fn struct_new_main() {
    // 関連関数newを使ってPerson構造体を初期化
    let taro = Person::new("太郎".to_string(), 18);
    // フィールド確認
    println!("{}さんは{}歳。", taro.name, taro.age);
}

// bmi_checker
// BMIb判定
#[test]
fn bmi_checker_main() {
    let body = Body2::new(163.0, 75.2, "田中");
    body.print_result();
    let body = Body2::new(158.2, 55.0, "鈴木");
    body.print_result();
    let body = Body2::new(174.2, 54.2, "井上");
    body.print_result();
}

// b判定用の構造体
struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    // オブジェクトを生成するメソッド
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange {
            min,
            max,
            label: label.to_string(),
        }
    }
    // 範囲内かテストする関数
    fn test(&self, v: f64) -> bool {
        (self.min <= v) && (v < self.max)
    }
}

// 身長と体重を表す構造体
struct Body2 {
    height: f64,  //cm
    weight: f64,  //kg
    name: String, //名前
}
impl Body2 {
    // オブジェクトを生成して返す
    fn new(height: f64, weight: f64, name: &str) -> Self {
        Body2 {
            height,
            weight,
            name: name.to_string(),
        }
    }
    // BMIを求める
    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powf(2.0)
    }
    // 肥満度測定を表示する
    fn print_result(&self) {
        // bmiを求める
        let bmi = self.calc_bmi();
        // 肥満度判定用のオブジェクトを配列で生成
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "低体重"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満1度"),
            BmiRange::new(30.0, 35.0, "肥満2度"),
            BmiRange::new(35.0, 40.0, "肥満3度"),
            BmiRange::new(40.0, 99.9, "肥満4度"),
        ];
        let mut result = String::from("不明");
        // 配列を1つずつテストする
        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }
        println!("{}さん, BMI={:.1}, 判定={}", self.name, bmi, result);
    }
}

// struct_copy_err/fix
struct Person2 {
    name: String,
    age: i32,
}
impl Person2 {
    fn new(name: &str, age:i32) -> Self {
        Self {name: name.to_string(), age}
    }
}

#[test]
fn struct_copy_err_fix_main() {
    // Taroを作成
    let taro = Person2::new("Taro", 18);
    // JiroはTaroを複製して名前だけ変えたい
    // let mut jiro = taro;
    // jiro.name = String::from("Jiro");
    let jiro = Person2 {
        name: String::from("Jiro"),
        ..taro  // 更新記法
    };
    // TaroとJiroを表示
    println!("{}, {}", taro.name, taro.age); // コメントアウトしたコードを使用するとここでエラーになる
    println!("{}, {}", jiro.name, jiro.age);
}