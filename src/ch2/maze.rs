use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    //乱数生成器を初期化
    let mut rng = rand::thread_rng();
    // 迷路を初期化。棒倒し法を使用
    let mut maze = [[0; MAP_N]; MAP_N];
    // 外周を壁にする
    for i in 0..MAP_N {
        maze[i][0] = 1;
        maze[i][MAP_N - 1] = 1;
        maze[0][i] = 1;
        maze[MAP_N - 1][i] = 1;
    }
    // 2マスに1つ壁を配置する
    let pitch = 2;
    for y in pitch..MAP_N - pitch {
        for x in pitch..MAP_N - pitch {
            if x % pitch != 0 || y % pitch != 0 {
                continue;
            }
            maze[y][x] = 1; // 壁にする

            // 上下左右の何れかを壁にする
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y - 1][x] = 1, // 上
                1 => maze[y + 1][x] = 1, // 下
                2 => maze[y][x - 1] = 1, // 左
                3 => maze[y][x + 1] = 1, // 右
                _ => {}                  // 何もしない
            }
        }
    }
    // 迷路を画面に出力
    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
