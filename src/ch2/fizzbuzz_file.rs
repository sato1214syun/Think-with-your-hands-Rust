use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    // 出力ファイル名
    let filename = "src/ch2/fizzbuzz_file_output.txt";
    // ファイル書き込みのブロックを作成(pythonのwith open as fp:と同じ)
    {
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp); // BuffWriteを使うことで書き込みが高速化される

        let mut result;
        for i in 0..=100 {
            if i % 3 == 0 && i % 5 == 0 {
                result = String::from("FizzBuzz");
            } else if i % 3 == 0 {
                result = String::from("Fizz");
            } else if i % 5 == 0 {
                result = String::from("Buzz");
            } else {
                result = format!{"{}", i};
            }
            writeln!(writer, "{}", result).unwrap();
            // writeln!を使わずに以下を使ったほうが若干高速
            // let line_byte = format!{"{}\n", &result};
            // writer.write(line_byte.as_bytes()).unwrap();
        }
    }
    println!("fizzbuzz result was output to {}", filename);
}
