# nfd_composer

macOS で作成されたファイルなどでよく見られる NFD（Unicode 正規化形式 D）のファイル名を NFC（正規化形式 C）に変換してリネームする CLI ツールです。

## インストール

```sh
git clone https://github.com/ironsand/nfd_composer.git
cd nfd_composer
cargo install --path .
```

バイナリは `~/.cargo/bin/` や `%USERPROFILE%\scoop\persist\rustup\.cargo\bin` などにインストールされます（rustup 環境では自動的に PATH が通っています）。

## 使い方

```sh
nfd_composer [--dry] [--verbose] <ファイルパス|パターン> [...]
```

### オプション

| オプション | 説明 |
|---|---|
| `--dry` | 実際にリネームせず、変換対象の一覧を表示します |
| `--verbose` | 既に NFC のファイルもスキップとして表示します |

### 例

```sh
# 単一ファイルを変換
nfd_composer 動画.mp4

# ワイルドカードで複数ファイルを一括変換
nfd_composer *.mp4

# ディレクトリ以下を再帰的に変換（ファイル→ディレクトリの順で処理）
nfd_composer **/*

# 実際にリネームする前に確認
nfd_composer --dry **/*
```

### 出力例

```
[OK]    動画.mp4 -> 動画.mp4
[DRY]   音楽.mp3 -> 音楽.mp3   # --dry 時
[SKIP]  既にNFCです: photo.jpg  # --verbose 時
[ERROR] ファイルが見つかりません: missing.txt
```

## ライセンス

MIT
