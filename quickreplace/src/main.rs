use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

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

/// 文字列から正規表現にマッチする部分を全て探し出し、それらを指定した文字に置き換える
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    // 最後に?を用いて、Regex::newが失敗した場合の処理を省略する
    let regex = Regex::new(target)?;
    // replace_allは元のテキストを指すポインタを返す。この場合は常に新しいコピーが必要なため、to_stringを作り
    // それをResult::Okでラップして返す
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    // 処理に使うデータを読み込む
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    // 読み込んだデータを元に、指定したターゲット文字列を置換した文字列を取得
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // 置換後のデータをファイルに書き出す
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };
}
