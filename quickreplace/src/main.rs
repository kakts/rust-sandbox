use text_colorizer::*;
use std::env;

// コマンドラインインタフェース
// #[derive..] で
// この構造体をprintln!マクロの{:?}フォーマットで出力できるようにするためのコードを出力するようにコンパイラに対して指示する
#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

/// 引数えラー時に出力するコマンドの利用例を表示する
fn print_usage() {
    // green()をつけることで、端末エミュレータ上で緑で出力するためのANSIエスケープコードが付加された文字列が生成される
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

/// 引数のパース
fn parse_args() -> Arguments {
    // 1番目の引数以外を文字列のベクタとして取得
    // 1番目の値は実行中のプログラム名
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
