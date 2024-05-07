use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    // 出力ファイル名
    let filename = "src/ch2/fizzbuzz_file2_output.txt";
    // ファイル書き込みのブロックを作成(pythonのwith open as fp:と同じ)
    {
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp); // BuffWriteを使うことで書き込みが高速化される

        let data = get_fizzbuzz(100);
        write!(writer, "{}", data).unwrap();
        // write!を使わずに以下を使ったほうが若干高速
        // writer.write_all(data.as_bytes()).unwrap();
    }
    println!("fizzbuzz result was output to {}", filename);
}

fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();
    for i in 0..=max {
        if i % 3 == 0 && i % 5 == 0 {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Buzz\n";
        } else {
            result += &format!{"{}\n", i};
        }
    }
    result
}
