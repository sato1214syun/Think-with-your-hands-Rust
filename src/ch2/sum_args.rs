fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut total = 0.0;

    for s in args {
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }

    println!("{}", total);
}