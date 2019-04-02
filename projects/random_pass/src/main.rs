#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate rand;

use docopt::Docopt;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod constants;

/// ヘルプメッセージを兼ねたコマンド宣言
const USAGE: &'static str = "
Random Password Generator.

Usage:
  create_pass create [--char=<symbol>] [--length=<len>]
  create_pass (-h | --help)

Options:
  -h --help        Show this screen.
  --char=<symbol>  Special Char.
  --length=<len>   Length [default: 0].
";

/// USAGE に合わせたArgs型
#[derive(Debug, Deserialize)]
pub struct Args {
    /// createコマンドフラグ（未使用）
    cmd_create: bool,
    /// 追加で使用する記号
    flag_char: Vec<String>,
    /// パスワードの長さ
    flag_length: i32,
}

/// パスワード作成処理
pub fn create_pass(args: Args) -> String {
    let mut pass_base: String = constants::PASS_PHRASE.to_string();
    for ch in &constants::SPECIAL_CHARS {
        if args.flag_char.contains(&ch.to_string()) {
            pass_base += &ch.to_string();
        }
    }
    let length: i32 = if args.flag_length > 0 {
        args.flag_length
    } else {
        constants::DEFAULT_PASS_LEN
    };
    let pass_base_list: Vec<char> = pass_base.chars().collect();
    let mut rng = thread_rng();
    let pass_str: Option<String> = (0..length)
        .map(|_| Some(*pass_base_list.choose(&mut rng)? as char))
        .collect();
    return pass_str.unwrap();
}

/// 実行処理
/// 
/// # Examples
/// 
/// 単純実行
/// 
/// ```sh
/// cargo run create
/// ```
/// 
/// パスフレーズ長変更
/// 
/// ```sh
/// cargo run create --length 24
/// ```
/// 記号を含める
/// 
/// ```sh
/// cargo run create --char "#.&"
/// ```
/// 
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    println!("{}", create_pass(args));
}
