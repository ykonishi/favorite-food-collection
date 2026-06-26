[wordcloud-rs](https://github.com/Inspirateur/wordcloud-rs) を使い `./users/<github-user-name>/list.txt` に列挙されているアイテムを収集し Wordcloud を作るプログラムを格納する．

# Latest Wordcloud

`main` ブランチが更新されるたびに GitHub Actions がワードクラウドを生成し，[Latest Wordcloud リリース](https://github.com/MS10-DP-ISSP/favorite-food-collection/releases/tag/latest) に公開する．

![Latest wordcloud](https://github.com/MS10-DP-ISSP/favorite-food-collection/releases/download/latest/wordcloud.png)

直接ダウンロード: https://github.com/MS10-DP-ISSP/favorite-food-collection/releases/download/latest/wordcloud.png

# Setup

## Install Rust

このコードベースは Rust で書かれているが，Git の使い方のハンズオンをするにあたっては Rust のセットアップをする必要は**ない**.

### macOS / Linux

[rustup](https://rustup.rs/) を使って Rust をインストールする．

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source "$HOME/.cargo/env"
$ rustc --version
```

### Windows

Visual C++ ビルドツールが必要な場合がある．

1. https://visualstudio.microsoft.com/visual-cpp-build-tools/ に移動し Download Build Tools をクリックしてインストールする

`WinGet` が使える環境では下記のコマンドで導入することもできる．

```PS
winget install -e --id Microsoft.VisualStudio.2022.BuildTools `
  --accept-source-agreements `
  --accept-package-agreements `
  --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

2. https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/setup?tabs=winget の説明に従う

```ps
PS> winget install Rustlang.Rustup
PS> rustup default stable
PS> rustc --version
```

## Generate Wordcloud

**リポジトリのルートディレクトリ**（この README.md がある場所）で実行する．

```sh
$ cargo run --release -- favorite-food --output wordcloud.png --width 1024 --height 1024
```

`users/*/list.txt` を収集し，ワードクラウド PNG を出力する．

### オプション

| オプション | 短縮 | デフォルト | 説明 |
|---|---|---|---|
| `--users-dir` | `-u` | `users` | `list.txt` を探すディレクトリ |
| `--output` | `-o` | `output/favorite-food.png` | 出力 PNG パス |
| `--width` | | `1024` | 画像幅（64 の倍数） |
| `--height` | | `1024` | 画像高さ（64 の倍数） |

```sh
$ cargo run --release -- favorite-food --output wordcloud.png --width 1024 --height 1024
```

### 実行時の注意

- **`--release` を付けることを推奨する** — `cargo run` のデフォルト（debug ビルド）はワードクラウド生成が数倍〜数十倍遅くなる．

### テスト

```sh
$ cargo test
```

## How to Contribute

1. このリポジトリを fork する．
2. fork したリポジトリをローカルマシン（あなたの作業用 PC）にクローンする．

```sh
$ git clone git@github.com:<your-github-user-name>/favorite-food-collection.git
$ cd favorite-food-collection
```

3. 自分の GitHub ユーザー名のディレクトリを作り，好きな食べ物を列挙する．

```sh
$ mkdir -p users/<your-github-user-name>
$ touch users/<your-github-user-name>/list.txt
```

### `list.txt` の書き方

- UTF-8 テキスト
- **1 行 = 1 食べ物**
- 空行は無視される
- `#` で始まる行はコメントとして無視される

例:

```
# 私の好きな食べ物
コーヒー
からあげ
炒飯
```

4. 変更を git に追加して commit する．

```sh
$ git add users/<your-github-user-name>/list.txt
$ git commit -m "add my favorite foods"
```

5. 自分の GitHub リポジトリに push する．

6. GitHub の画面から本家リポジトリ（`MS10-DP-ISSP/favorite-food-collection`）に Pull Request を作成する．

複数人が同じ食べ物を書いた場合，出現回数が多いほどワードクラウド上で大きく表示される．

### 日本語フォントについて

日本語の表示には，OS にインストールされた日本語対応フォント（ヒラギノ角ゴシック，Noto Sans CJK など）を自動的に使用する．フォントが見つからない環境では文字化けする可能性がある．
