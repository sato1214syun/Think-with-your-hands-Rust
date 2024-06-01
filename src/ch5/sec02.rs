// 波形合成で音楽を奏でよう
//ダミーのmain関数
fn main() {}

// 波形合成と再生
// sinewave
use hound::WavWriter;
use std::f32::consts::PI;

// 定数を宣言
const SAMPLE_RATE: u32 = 44100;
const TONE: f32 = 440.0; // 440Hz = A
#[test]
fn sinewave_main() {
    // WAVファイルのフォーマットを指定
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    // WavWriterのオブジェクトを作成
    let mut fw = WavWriter::create("./src/ch5/sec02_result/a.wav", spec).unwrap();
    // 連続でサイン波を書き込む
    let samples = SAMPLE_RATE * 3; // 3秒
    for t in 0..samples {
        let v = ((t as f32 / SAMPLE_RATE as f32) * TONE * 2.0 * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

// 音階を奏でてみよう
// sine_melody
// use std::f32::consts::PI;
use std::io::{Seek, Write};
// use hound::WavWriter;

// 定数
const SAMPLE_RATE2: f32 = 44100.0;
const BPM: f32 = 120.0; // BPMテンポ

#[test]
#[allow(unused_variables)]
fn sine_melody_main() {
    // Wavオブジェクトを生成
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE2 as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut fw = WavWriter::create("./src/ch5/sec02_result/melody.wav", spec).unwrap();
    // 音階を定義
    let (c4, d4, e4, _f4) = (261.626, 293.665, 329.638, 349.228);
    let (g4, _a4, _b4, _c5) = (391.995, 440.000, 493.883, 523.251);
    // 音長を定義
    let l4 = ((60.0 / BPM) * SAMPLE_RATE2) as u32; // 四分音符
    let l2 = l4 * 2; // 二分音符

    // メロディーを指定
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l2);
}

// 指定の音階を指定の長さ分書き込む
fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32)
where
    W: Write + Seek,
{
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE2;
        let v = (a * tone * 2.0 * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

// MML(Music MAcro Language)コンパイラーを作る
/*
- "cdefgab": ドレミファソラシに変換して演奏する
- r: 休符
- l: 音長指定。l4(四分音符), l2(二分音符), l1(全音符)
- o: オクターブ変更
*/
// mml
// mml_parser.rsとwav_writer.rsを別途準備

mod mml_lib;
use mml_lib::mml_parser;
use mml_lib::wav_writer;

#[test]
fn mml_main() {
    // カエルの歌の楽譜
    let mml = format!(
        "{}{}",
        "o5 l4 cdef edl2c l4 efga gfl2e",
        "l4 crcr crcr l8 ccdd eeff l4 ed l2c",
    );
    // 楽譜を一音ずつのVec<Note>に変換
    let notes = mml_parser::parse(mml);
    // WAVファイルに書き出し
    let bpm = 120.0;
    wav_writer::write("./src/ch5/sec02_result/kaeru.wav", notes, bpm);

    // キラキラ星の楽譜
    let mml = format!(
        "{}{}{}",
        "o5 l4 ccgg aal2g l4 ffee ddl2c",
        "l4 ggff eel2d l4 ggff eel2d",
        "l4 ccgg aal2g l4 ffee ddl2c",
    );
    // 楽譜を一音ずつのVec<Note>に変換
    let notes = mml_parser::parse(mml);
    // WAVファイルに書き出し
    let bpm = 120.0;
    wav_writer::write("./src/ch5/sec02_result/kirakira.wav", notes, bpm);
}
