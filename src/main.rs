use std::env;
use std::ffi::OsStr;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

fn is_nfd(s: &str) -> bool {
    let nfc: String = s.chars().nfc().collect();
    nfc != s
}

fn to_nfc(s: &str) -> String {
    s.chars().nfc().collect()
}

fn process_path(path_str: &str, dry_run: bool, verbose: bool) {
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!("[ERROR] ファイルが見つかりません: {}", path_str);
        return;
    }

    let file_name = match path.file_name().and_then(OsStr::to_str) {
        Some(name) => name,
        None => {
            eprintln!("[ERROR] ファイル名を取得できません: {}", path_str);
            return;
        }
    };

    if !is_nfd(file_name) {
        if verbose {
            println!("[SKIP]  既にNFCです: {}", path_str);
        }
        return;
    }

    let nfc_name = to_nfc(file_name);
    let new_path = match path.parent() {
        Some(parent) => parent.join(&nfc_name),
        None => Path::new(&nfc_name).to_path_buf(),
    };

    if dry_run {
        println!("[DRY]   {} -> {}", file_name, nfc_name);
        return;
    }

    match std::fs::rename(path, &new_path) {
        Ok(()) => println!("[OK]    {} -> {}", file_name, nfc_name),
        Err(e) => eprintln!("[ERROR] リネーム失敗 {}: {}", path_str, e),
    }
}

fn expand_paths(patterns: Vec<&String>) -> Vec<String> {
    let mut result = Vec::new();
    for pattern in patterns {
        match glob::glob(pattern) {
            Ok(entries) => {
                let mut matched = false;
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            if let Some(s) = path.to_str() {
                                result.push(s.to_string());
                                matched = true;
                            }
                        }
                        Err(e) => eprintln!("[ERROR] グロブ展開エラー: {}", e),
                    }
                }
                if !matched {
                    result.push(pattern.to_string());
                }
            }
            Err(_) => result.push(pattern.to_string()),
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let dry_run = args.iter().any(|a| a == "--dry");
    let verbose = args.iter().any(|a| a == "--verbose");
    let patterns: Vec<&String> = args.iter().filter(|a| *a != "--dry" && *a != "--verbose").collect();

    if patterns.is_empty() {
        eprintln!("使い方: nfd_composer [--dry] [--verbose] <ファイルパス|パターン> [...]");
        eprintln!("  ファイル名がNFDの場合、NFCに変換してリネームします。");
        eprintln!("  --dry      実際にリネームせず、変換対象を表示します。");
        eprintln!("  --verbose  スキップしたファイルも表示します。");
        std::process::exit(1);
    }

    let mut paths = expand_paths(patterns);
    paths.sort_by(|a, b| {
        let depth_a = Path::new(a).components().count();
        let depth_b = Path::new(b).components().count();
        depth_b.cmp(&depth_a)
    });
    for path in &paths {
        process_path(path, dry_run, verbose);
    }
}
