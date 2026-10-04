# count_words の引数を参照に変え、文字列の複製をなくす

## 概要

`count_words` が、ファイルの文字列を複製せずに借りて数えるように変えました。
大きなファイルを数えるときの待ち時間を減らすための変更です。コマンドの引数と出力は変えていません。

## 背景

変更前は、`main` がファイル全体の文字列を `clone` してから `count_words` に渡していました。
`main` は、渡したあとで行数を数えるために、同じ文字列をもう一度使います。`clone` を外すと、所有権のエラー E0382 でコンパイルできませんでした。
そのため、大きなファイルほど、複製の時間とメモリが余分にかかっていました。

## 変更内容

変更したファイルは `src/main.rs` だけです。

- `count_words`: 引数を `String` から `&str` に変えた。戻り値のキーも `String` から `&str` に変え、単語ごとの文字列の生成をなくした
- `rank`: 引数と戻り値の単語の型を `&str` に合わせた
- `main`: `count_words(text.clone())` を `count_words(&text)` に変えた
- テスト: 3件の引数と期待値を、新しい型に合わせた

## 確認方法

`cargo test` を実行し、3件とも通りました。

```text
running 3 tests
test tests::empty_text_has_no_words ... ok
test tests::rank_orders_by_count_then_word ... ok
test tests::counts_repeated_words ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

手では、次の2つを確かめました。

- `wordfreq sample.txt --top 3` の出力が、変更の前後で同じである
- 27MB のファイルで、処理時間の中央値が 360ms から 202ms になった。条件と値は [report.md](report.md) にある

`cargo clippy` は、警告を1件出します。内容は「レビューで見てほしい点」に書きました。

## 影響範囲

コマンドの書式、出力の形式、終了コードは変わりません。
ファイルを読めないときに panic する動き（[issue-bug.md](issue-bug.md)）にも触れていません。別の変更で直します。

## レビューで見てほしい点

`rank` に、ライフタイムの注釈 `'a` を書いています。

```rust
fn rank<'a>(counts: HashMap<&'a str, usize>) -> Vec<(&'a str, usize)> {
```

clippy は、この注釈を省略できると警告します（`needless_lifetimes`）。
戻り値の単語が引数の単語を借りていることを読み取りやすくするために、注釈を残しました。省略する形にそろえるほうがよければ、指摘してください。
