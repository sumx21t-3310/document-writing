# 所有権のエラー E0382 を、参照を渡す形に直す

関数に渡した値を、呼び出しのあとでもう一度使うと、Rust のコンパイラは E0382 を出します。
この手順では、関数の引数を参照に変えて、E0382 を消します。関数が値を読むだけの場合に使えます。

## この手順でできること

手順を終えると、`cargo run` が通ります。関数に渡した値を、呼び出しのあとでも使えます。

## 前提条件

- Rust がインストールされていること。この手順は Rust 1.95.0 で確かめている
- エラーの1行目が `error[E0382]: borrow of moved value` であること
- エラーの中に `consider changing this parameter type in function` で始まる `note` があること
- 関数が、受け取った値を読むだけであること。値を保存したり、そのまま返したりする関数は対象外である
- 関数のソースコードを編集できること

この手順では、次のコードを例にします。`shout` は、受け取った文字列を大文字にして返す関数です。

```rust
fn shout(text: String) -> String {
    text.to_uppercase()
}

fn main() {
    let name = String::from("ferris");
    let loud = shout(name);
    println!("{name} -> {loud}");
}
```

`cargo run` を実行すると、次のエラーが出ます。

```text
error[E0382]: borrow of moved value: `name`
 --> src\main.rs:8:16
  |
6 |     let name = String::from("ferris");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
7 |     let loud = shout(name);
  |                      ---- value moved here
8 |     println!("{name} -> {loud}");
  |                ^^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `shout` to borrow instead if owning the value isn't necessary
 --> src\main.rs:1:16
  |
1 | fn shout(text: String) -> String {
  |    -----       ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
7 |     let loud = shout(name.clone());
  |                          ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `shout` (bin "shout") due to 1 previous error
```

## 手順1: エラーから、直す関数を読み取る

エラーの `note` にある `in this function` の行を探します。その行が指している関数が、直す対象です。
例では、1行目の `shout` です。

あわせて、次の3か所を読むと、何が起きたかが分かります。

- `move occurs because`: 値を持っている変数（6行目の `name`）
- `value moved here`: 値が関数へ移った場所（7行目）
- `value borrowed here after move`: 移ったあとで値を使った場所（8行目）

## 手順2: 関数の引数の型を、参照に変える

手順1で見つけた関数の引数の型を、`String` から `&str` に書き換えます。

```rust
fn shout(text: &str) -> String {
    text.to_uppercase()
}
```

この時点で `cargo build` を実行すると、E0382 が消え、代わりに呼び出しの行を指す E0308 が出ます。

```text
error[E0308]: mismatched types
 --> src\main.rs:7:22
  |
7 |     let loud = shout(name);
  |                ----- ^^^^ expected `&str`, found `String`
  |                |
  |                arguments to this function are incorrect
  |
note: function defined here
 --> src\main.rs:1:4
  |
1 | fn shout(text: &str) -> String {
  |    ^^^^^ ----------
help: consider borrowing here
  |
7 |     let loud = shout(&name);
  |                      +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `shout` (bin "shout") due to 1 previous error
```

## 手順3: 呼び出しで、値の前に `&` を付ける

E0308 が指している呼び出しを、`shout(name)` から `shout(&name)` に書き換えます。

```rust
    let loud = shout(&name);
```

## 結果を確かめる

`cargo run` を実行します。

```bash
cargo run
```

エラーが出ずに、次の行が表示されれば終わりです。

```text
ferris -> FERRIS
```

## うまくいかないとき

手順2のあとで、関数の中を指す別のエラーが出たときは、関数が値を保存しているか、そのまま返しています。
その関数は、この手順の対象外です。[所有権の解説](explanation.md) を読み、値をどの変数に持たせるかを決め直してください。

別の行を指す E0382 が残っているときは、同じ値を渡している関数がほかにもあります。残ったエラーについて、手順1からもう一度進めてください。
