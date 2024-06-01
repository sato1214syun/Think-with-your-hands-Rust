// 80年代ゲーム音楽を作ってみよう
//ダミーのmain関数
fn main() {}

// ノコギリ波で演奏するプログラム
// sawtooth = t / (サンプルレート / 周波数) % 1.0
// sawwave

use hound;
const SAMPLE_RATE: f32 = 44100.0;

// 波形を生成
fn gen_wave(shape: &str, note_no: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = note_no_to_hz(note_no); // 周波数を得る
    let form_samples = SAMPLE_RATE / tone; // 周期を得る
    let mut wav: Vec<f32> = vec![0.0; len];
    let gen_wave_func: Box<dyn Fn(f32, usize) -> f32>;
    match shape {
        "sawtooth" => gen_wave_func = Box::new(sawtooth_wave),
        "square" => gen_wave_func = Box::new(square_wave),
        "triangle" => gen_wave_func = Box::new(tri_wave),
        _ => panic!("use \"sawtooth\" or \"square\" or \"triangle\" as shape"),
    }
    for i in 0..len {
        wav[i] = gen_wave_func(form_samples, i);
    }
    // 音量を調整する
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// ノコギリ波を生成する
fn sawtooth_wave(form_samples: f32, i: usize) -> f32 {
    let pif = (i as f32 / form_samples) % 1.0;
    pif * 2.0 - 1.0
}

// 矩形波を生成する
fn square_wave(form_samples: f32, i: usize) -> f32 {
    let half_fs = (form_samples / 2.0) as usize;
    let hl = (i / half_fs) % 2;
    if hl == 0 {
        -1.0
    } else {
        1.0
    }
}

// 三角波を生成する
fn tri_wave(form_samples: f32, i: usize) -> f32 {
    let half_fs = form_samples / 2.0;
    let hi = i as f32 / half_fs;
    let v: f32 = 2.0 * (hi % 1.0) - 1.0;
    let is_climbing = hi.floor() as usize % 2 == 0;
    if is_climbing {
        v
    } else {
        -v
    }
}

#[test]
fn sawwave_main() {
    //WavWriterを生成
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("./src/ch5/sec03_result/saw.wav", spec).unwrap();
    //ノコギリ波を生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    // wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    // wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));
    wav.extend(gen_wave("sawtooth", 60, calc_len(bpm, 4), 0.5));
    wav.extend(gen_wave("sawtooth", 64, calc_len(bpm, 4), 0.5));
    wav.extend(gen_wave("sawtooth", 67, calc_len(bpm, 4), 0.5));
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        // println!("{}", v);
    }
}

// ノート番号から周波数へ変換
fn note_no_to_hz(no: i32) -> f32 {
    440.0 * 2.0f32.powf((no - 69) as f32 / 12.0)
}
// n音符のサンプル数を計算
fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / n as f32) * base_len) as usize
}

// ノコギリ波を生成する
// fn sawtooth_wave(note_no: i32, len: usize, gain: f32) -> Vec<f32> {
//     let tone = note_no_to_hz(note_no); // 周波数を得る
//     let form_samples = SAMPLE_RATE / tone; // 周期を得る
//     let mut wav: Vec<f32> = vec![0.0; len];
//     for i in 0..len {
//         let pif = (i as f32 / form_samples) % 1.0;
//         wav[i] = pif * 2.0 - 1.0;
//     }
//     // 音量を調整する
//     wav.into_iter().map(|v| (v * gain) as f32).collect()
// }

// 矩形波を生成するプログラム
// squarewave
// use hound;
// const SAMPLE_RATE: f32 = 44100.0;
#[test]
fn squarewave_main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("./src/ch5/sec03_result/sq.wav", spec).unwrap();
    // 矩形波を生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // メロディーを生成
    [60, 64, 67, 64, 60, 64, 67, 72].iter().for_each(|no| {
        // wav.extend(square_wave(*no, calc_len(bpm, 8), 0.5));
        wav.extend(gen_wave("square", *no, calc_len(bpm, 8), 0.5));
    });
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}

// // ノート番号から周波数へ変換
// fn note_no_to_hz(no: i32) -> f32 {
//     440.0 * 2.0f32.powf((no - 69) as f32 / 12.0)
// }

// n音符のサンプル数を計算
// fn calc_len(bpm: usize, n: usize) -> usize {
//     let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
//     ((4.0 / n as f32) * base_len) as usize
// }

// 矩形波を生成する
// fn square_wave(note_no: i32, len: usize, gain: f32) -> Vec<f32> {
//     let tone = note_no_to_hz(note_no); // 周波数を得る
//     let form_samples = SAMPLE_RATE / tone; // 周期を得る
//     let mut wav: Vec<f32> = vec![0.0; len];
//     let half_fs = (form_samples / 2.0) as usize;
//     for i in 0..len {
//         let hl = (i / half_fs) % 2;
//         wav[i] = if hl == 0 { -1.0 } else { 1.0 };
//     }
//     // 音量を調整する
//     wav.into_iter().map(|v| (v * gain) as f32).collect()
// }

// 三角波を生成する
// trianglewave
#[test]
fn trianglewave_main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("./src/ch5/sec03_result/tri.wav", spec).unwrap();
    // 三角波を生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // メロディーを生成
    [60, 64, 67, 64, 60, 64, 67, 72].iter().for_each(|no| {
        // wav.extend(square_wave(*no, calc_len(bpm, 8), 0.5));
        wav.extend(gen_wave("triangle", *no, calc_len(bpm, 8), 0.5));
    });
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}

// fn tri_wave(note_no: i32, len: usize, gain: f32) -> Vec<f32> {
//     let tone = note_no_to_hz(note_no); // 周波数を得る
//     let form_samples = SAMPLE_RATE / tone; // 周期を得る
//     let mut wav: Vec<f32> = vec![0.0; len];
//     let half_fs = (form_samples / 2.0);
//     for i in 0..len {
//         let hi = i as f32 / half_fs;
//         let mut v: f32 = 2.0 * (hi % 1.0) - 1.0;
//         let is_climbing = hi.floor() as usize % 2 == 0;
//         v = if is_climbing { v } else { -v };
//         wav[i] = v;
//     }
//     // 音量を調整する
//     wav.into_iter().map(|v| (v * gain) as f32).collect()
// }

// ホワイトノイズを生成する
// noise
use rand::prelude::*;
#[test]
fn noise_main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("./src/ch5/sec03_result/noise.wav", spec).unwrap();
    // ホワイトノイズ生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // 生成
    // フルレンジのノイズを生成
    wav.extend(noise(2.0, -1.0, calc_len(bpm, 2)));
    // 0.8~1.0
    wav.extend(noise(0.2, 0.8, calc_len(bpm, 2)));
    // -1.0~-0.2
    wav.extend(noise(0.8, -1.0, calc_len(bpm, 2)));
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
// ノイズを生成する
fn noise(range: f32, shift: f32, len: usize) -> Vec<f32> {
    let mut wav: Vec<f32> = vec![0.0; len];
    let mut rng = rand::thread_rng();
    for i in 0..len {
        wav[i] = rng.gen::<f32>() * range + shift;
    }
    // 音量を調整
    let gain = 0.5;
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// パルス波
#[test]
fn pulsewave_main() {
    // WavWriterを生成
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(
        "./src/ch5/sec03_result/pulse.wav", spec).unwrap();
    // 波形をwavに追加する
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // Duty比を変えてパルス波を生成 --- (*1)
    [0.3, 0.1, 0.7, 0.5].iter().for_each(|duty| {
        [60,64,67,72].iter().for_each(|no| {
            wav.extend(pulse(
                *no, calc_len(bpm, 4), 0.5, *duty));
        });
    });
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
// パルス波を生成する --- (*2)
fn pulse(no: i32, len: usize, gain: f32, duty: f32) -> Vec<f32> {
    let tone = note_no_to_hz(no); // 周波数
    let form_samples = SAMPLE_RATE / tone; // 周期
    let mut wav:Vec<f32> = vec![0.0; len];
    for i in 0..len {
        let saw = (i as f32 / form_samples) % 1.0;
        wav[i] = if saw > duty { -1.0 } else { 1.0 };
    }
    // 音量を調節する
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// FM音源
// fm_synth
mod fm_synth_lib;
use fm_synth_lib::fm;
#[test]
fn fm_synth_main() {
    // WavWriterを生成
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: fm::SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(
        "./src/ch5/sec03_result/fm.wav", spec).unwrap();
    // 波形をtrackに追加する --- (*1)
    let mut track: Vec<f32> = vec![];
    let bpm = 120;
    let len = fm::calc_len(bpm, 4);
    // 音色のパラメータを変えながら演奏 --- (*2)
    let params = [(4.5,2.0),(7.0,3.0),(3.0,2.0),(11.0,4.0)];
    for p in params {
        for note_no in [60,64,67,64, 60,64,67,72] {
            fm::write(&mut track, fm::Note {
                no: note_no, len, gain: 0.5, params: p
            });
        }
    }
    // ファイルに書き込む
    for v in track.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}