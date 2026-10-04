# 存在しないファイルを渡すと panic し、原因を読み取りにくいメッセージが出る

## 概要

`wordfreq` に存在しないファイルのパスを渡すと、panic して終了コード 101 で終わります。
表示されるメッセージは Rust の内部の値をそのまま出した形で、利用者が「ファイルがない」と読み取るのに時間がかかります。

## 期待した動き

- ファイルを読めなかったことと、渡したパスが、1行のメッセージで分かること
- panic せずに終わること

## 実際の動き

標準エラー出力に次の3行が出て、終了コード 101 で終わります。`(14880)` の数字は、実行のたびに変わります。

```text
thread 'main' (14880) panicked at src\main.rs:40:41:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## 再現手順

1. このリポジトリの `samples/wordfreq` フォルダへ移動する
2. `cargo build --release` を実行する
3. `nofile.txt` という名前のファイルが、フォルダにないことを確かめる
4. `cargo run -q --release -- nofile.txt` を実行する

5回試して、5回とも同じ動きになりました。

## 環境

| 項目 | 値 |
| :-- | :-- |
| wordfreq | 0.1.0 |
| Rust | 1.95.0 |
| OS | Windows 11 |

## 影響範囲と回避策

ファイルのパスを打ち間違えた利用者の全員が、このメッセージを見ます。
スクリプトから呼ぶ人は、終了コード 101 を「ファイルを読めなかった」と判定することになります。

回避策は、実行する前に、ファイルがあることを確かめることです。

同じ動きは、フォルダのパスを渡したときと、UTF-8 として読めないファイルを渡したときにも起きます。どちらも終了コードは 101 で、メッセージの2行目だけが次のように変わります。

```text
called `Result::unwrap()` on an `Err` value: Os { code: 5, kind: PermissionDenied, message: "アクセスが拒否されました。" }
```

```text
called `Result::unwrap()` on an `Err` value: Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }
```

## 原因

メッセージが指している `src/main.rs` の40行目は、次のコードです。

```rust
    let text = fs::read_to_string(path).unwrap();
```

ここからは推測です。`unwrap` は、読み込みが失敗すると panic します。失敗したときの処理を書いていないことが原因だと考えられます。
直し方の方針は、[決定の記録](decision.md) にあります。
