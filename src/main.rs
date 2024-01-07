use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn pause() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn main() {
    println!("记忆力测试.记住接下来出现的数字.\n");
    pause();
    let mut difficult: u8 = 1;
    let mut rng = rand::thread_rng();
    'game_loop: loop {
        let mut secret_numbers = vec![];
        for _ in 0..difficult {
            let random_number = loop {
                let num = rng.gen_range(10..100);
                if secret_numbers.len() == 0 || num != secret_numbers[secret_numbers.len() - 1] { break num; }
            };
            print!("{random_number}\r");
            io::stdout().flush().expect("Flush output fail");
            sleep(Duration::from_secs(1));
            secret_numbers.push(random_number);
        }
        let mut index = 0;
        println!("输入刚刚出现的数字(一行一个):");
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Fail to read line.");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            match guess.cmp(&secret_numbers[index]) {
                Ordering::Equal => {
                    index += 1;
                    if index == secret_numbers.len() {
                        println!("================");
                        break;
                    };
                }
                _ => {
                    println!("正确答案是: {:?}", secret_numbers);
                    break 'game_loop;
                }
            }
        }
        difficult += 1;
    }
    println!("你的得分是: {difficult}");
    pause();
}
