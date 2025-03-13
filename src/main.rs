use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal,
};
use rand::Rng;

struct PlayerPosition {
    x: usize,
    y: usize,
}

fn matrix_create(matrix: &Vec<Vec<String>>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols {
            print!("{}", matrix[i][j]);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        if i < rows {
            println!("");
        }
    }
}

fn matrix_clear(rows: usize) {
    for _ in 0..rows {
        print!("\x1b[1A");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        print!("\x1b[2K");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
    print!("\r");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
fn main() {
    let rows = 10;
    let cols = 10;
    let mut matrix: Vec<Vec<String>> = (0..rows).map(|_| vec!["・".to_string(); cols]).collect();
    let mut player: PlayerPosition = PlayerPosition {
        x: rand::rng().random_range(0..cols) as usize,
        y: rand::rng().random_range(0..rows) as usize,
    };
    println!("Press 'h', 'j', 'k', or 'l' (Press 'q' to quit)");
    println!("Left (h)");
    println!("Down (j)");
    println!("Up (k)");
    println!("Right (l)");
    matrix[player.y][player.x] = "＠".to_string();

    matrix_create(&matrix, rows, cols);
    // ターミナルを非カノニカルモード（キー入力を即時取得）にする
    terminal::enable_raw_mode().unwrap();

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('h') => {
                        if player.x != 0 {
                            matrix[player.y][player.x] = "・".to_string();
                            player.x -= 1;
                        }
                    }
                    KeyCode::Char('j') => {
                        if player.y != rows - 1 {
                            matrix[player.y][player.x] = "・".to_string();
                            player.y += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if player.y != 0 {
                            matrix[player.y][player.x] = "・".to_string();
                            player.y -= 1;
                        }
                    }
                    KeyCode::Char('l') => {
                        if player.x != cols - 1 {
                            matrix[player.y][player.x] = "・".to_string();
                            player.x += 1;
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
                matrix[player.y][player.x] = "＠".to_string();
                matrix_clear(rows);
                matrix_create(&matrix, rows, cols);
            }
        }
    }

    // ターミナルのモードを戻す
    terminal::disable_raw_mode().unwrap();
    println!("Finish!");
}
