use std::{path::Path, time::{self, UNIX_EPOCH}, fs};

use chrono::Local;
use clap::Parser;

/// 删除指定路径下面的所有文件
#[derive(Parser, Debug)]
#[clap(
    name = "rmf",
    version = "1.0.0",
    author = "作者: 技安 <tgxz2000@gmail.com>",
    about = "删除指定路径下面的所有文件, 帮助输入 `-h`",
    long_about = None,
    arg_required_else_help(true)
)]
struct Cli {
    /// 要删除的文件上级路径
    #[clap(short, long)]
    path: String,

    /// 保留最近文件的天数
    #[clap(short, long, default_value = "1")]
    days: u32,
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(&cli.path);
    if !path.is_dir() {
        println!("{} is not a directory", cli.path);
        return;
    }

    // 开始时间
    let start = time::Instant::now();

    // 现在的时间
    let timestamp = if cli.days > 0 {
        Local::today().and_hms(0, 0, 0).timestamp() - ((cli.days as i64 - 1) * 24 * 60 * 60)
    } else {
        Local::now().timestamp() + 100
    };

    for dir_entry in path.read_dir()
        .expect("Unable to read directory").
        into_iter() {
        if let Ok(dir_entry) = dir_entry {
            if let Ok(metadata) = dir_entry.metadata() {
                if let Ok(time) = metadata.modified() {
                    if let Ok(time) = time.duration_since(UNIX_EPOCH) {
                        if (time.as_secs() as i64) < timestamp {
                            if metadata.is_file() {
                                fs::remove_file(dir_entry.path()).expect("Unable to remove file");
                            } else if metadata.is_dir() {
                                fs::remove_dir_all(dir_entry.path()).expect("Unable to remove directory");
                            }
                        }
                    }
                }
            }
        }
    }

    // 用时
    let elapsed = start.elapsed();
    println!("duration: {:?}", elapsed);
}
