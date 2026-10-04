# Rust で最初のコマンドを作る: ファイルの行数を数える

このチュートリアルでは、ファイルの行数を表示するコマンド `linecount` を Rust で作ります。
Rust を初めて使う人が対象です。ターミナルでコマンドを実行できれば、最後まで進められます。

## このチュートリアルで作るもの

完成すると、ファイル名を渡して行数を表示できます。

```text
memo.txt: 3 行
```

作りながら、次の3つを覚えます。

- `cargo new` でプロジェクトを作る
- `cargo run` でプログラムを動かす
- ファイルを読み、コマンドの引数を受け取る

## 準備

Rust を [公式サイトの手順](https://www.rust-lang.org/tools/install) でインストールします。
終わったら、ターミナルで次のコマンドを実行します。

```bash
cargo --version
```

バージョンが表示されれば、準備は終わりです。この手順は、次のバージョンで確かめています。

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

## 手順1: プロジェクトを作る

`cargo` は、Rust のプロジェクトを作り、ビルドし、実行する道具です。次のコマンドでプロジェクトを作ります。

```bash
cargo new linecount
```

次の行が表示されれば、`linecount` フォルダができています。

```text
    Creating binary (application) `linecount` package
```

できたフォルダへ移動します。

```bash
cd linecount
```

フォルダの中には、設定を書く `Cargo.toml` と、プログラムを書く `src/main.rs` があります。

## 手順2: そのまま動かす

`cargo new` は、あいさつを表示するプログラムを最初から用意しています。何も書き換えずに実行します。

```bash
cargo run
```

出力の最後の2行が次のようになれば、ビルドと実行ができています。`Running` の行は Windows での表示です。

```text
     Running `target\debug\linecount.exe`
Hello, world!
```

## 手順3: ファイルの行数を数える

数える対象のファイルを用意します。`linecount` フォルダの直下に `memo.txt` を作り、次の3行を書いて保存します。

```text
apple
banana
cherry
```

`src/main.rs` の中身を、次のコードにすべて置き換えます。

```rust
use std::fs;

fn main() {
    let text = fs::read_to_string("memo.txt").expect("memo.txt を読めませんでした");
    println!("{} 行", text.lines().count());
}
```

`fs::read_to_string` で、ファイルの中身を文字列として読み込みます。`text.lines().count()` で、その文字列の行数を数えます。
もう一度実行します。

```bash
cargo run
```

最後の行に行数が表示されます。

```text
3 行
```

## 手順4: ファイル名を引数で受け取る

ここまでのプログラムは、`memo.txt` しか数えられません。ファイル名をコマンドの引数で受け取る形に変えます。
`src/main.rs` を、次のコードにすべて置き換えます。

```rust
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("ファイル名を指定してください");
    let text = fs::read_to_string(&path).expect("ファイルを読めませんでした");
    println!("{path}: {} 行", text.lines().count());
}
```

`env::args().nth(1)` で、コマンドの1つ目の引数を取り出します。`&path` の `&` の意味は、あとで [解説](explanation.md) を読むと分かります。ここではそのまま書き写してください。

ファイル名を付けて実行します。`--` より後ろが、プログラムに渡す引数です。

```bash
cargo run -- memo.txt
```

最後の行に、ファイル名と行数が表示されます。

```text
memo.txt: 3 行
```

## 完成したものを動かす

別のファイルでも数えられることを確かめます。プロジェクトの `Cargo.toml` を渡します。

```bash
cargo run -- Cargo.toml
```

次の行が表示されれば完成です。

```text
Cargo.toml: 6 行
```

## 次に読むもの

- [所有権: Rust が値の持ち主を1つに決める理由](explanation.md): 手順4の `&` の意味を説明している
- [所有権のエラー E0382 を、参照を渡す形に直す](how-to.md): コードを書き換えていて、最初に出会いやすいエラーの直し方
- [The Rust Programming Language](https://doc.rust-lang.org/book/): Rust の公式の入門書
