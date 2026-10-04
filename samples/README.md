# サンプル

テーマファイルごとに1本ずつ、このスキルで書いた文書を置いています。
題材は、読みにくいと言われることの多い Rust にそろえました。
文書の種類ごとの見出しの並びと、書き上がりの文体を確かめるときに開いてください。

## サンプルの一覧

| テーマファイル | サンプル | 内容 |
| :-- | :-- | :-- |
| `tutorial` | [tutorial.md](tutorial.md) | ファイルの行数を数えるコマンドを、Rust で最初から作る |
| `how-to` | [how-to.md](how-to.md) | 所有権のエラー E0382 を、参照を渡す形に直す |
| `reference` | [reference.md](reference.md) | `wordfreq` コマンドの引数、出力、終了コード |
| `explanation` | [explanation.md](explanation.md) | Rust が値の持ち主を1つに決める理由 |
| `readme` | [wordfreq/README.md](wordfreq/README.md) | `wordfreq` の README |
| `report` | [report.md](report.md) | 引数を参照に変えた前後の、処理時間の計測結果 |
| `proposal` | [proposal.md](proposal.md) | テストと clippy を CI で流す提案 |
| `pull-request` | [pull-request.md](pull-request.md) | 引数を参照に変える変更の PR 本文 |
| `issue-bug` | [issue-bug.md](issue-bug.md) | 存在しないファイルを渡すと panic する不具合の報告 |
| `issue-feature` | [issue-feature.md](issue-feature.md) | 標準入力から読めるようにする要望 |
| `decision` | [decision.md](decision.md) | ファイルを読めないときの終わり方を決めた記録 |

## サンプルの前提

`wordfreq` は、サンプルのために作った小さなコマンドです。ソースコードは [wordfreq/](wordfreq/) にあり、実際にビルドして動かせます。

- コマンドの出力、エラーメッセージ、計測値は、2026-10-05 に Windows 11 と Rust 1.95.0 で実際に流した結果である
- issue、PR、提案書、決定の記録は、文書の型を示すためのサンプルで、GitHub には投稿していない。文書どうしの参照は、このフォルダのファイルへのリンクで書いている
- [issue-bug.md](issue-bug.md) の不具合は、再現手順を試せるように、直さずに残している
- 変更前のコードはリポジトリに置いていない。[report.md](report.md) と [pull-request.md](pull-request.md) に、変更前の該当箇所を載せている
