# サンプル

テーマファイルごとに1本ずつ、このスキルで書いた文書を置いています。
題材は、エージェント用のスキル（`SKILL.md`）の作り方にそろえました。
文書の種類ごとの見出しの並びと、書き上がりの文体を確かめるときに開いてください。

## サンプルの一覧

| テーマファイル | サンプル | 内容 |
| :-- | :-- | :-- |
| `tutorial` | [tutorial.md](tutorial.md) | 最初のスキルを作り、プロジェクトに入れる |
| `how-to` | [how-to.md](how-to.md) | `SKILL.md` の frontmatter を、YAML として読める形に直す |
| `reference` | [reference.md](reference.md) | `SKILL.md` の frontmatter の項目と、分量の目安 |
| `explanation` | [explanation.md](explanation.md) | `description` が、スキルが呼ばれるかどうかを決める理由 |
| `readme` | [../README.md](../README.md) | このリポジトリの README |
| `report` | [report.md](report.md) | `npx skills add` でこのスキルを入れられるかを確かめた結果 |
| `proposal` | [proposal.md](proposal.md) | 公開前の手順に、発火テストを入れる提案 |
| `pull-request` | [pull-request.md](pull-request.md) | `description` をクォートで囲む変更の PR 本文 |
| `issue-bug` | [issue-bug.md](issue-bug.md) | frontmatter を YAML として読めず、スキルが見つからない不具合の報告 |
| `issue-feature` | [issue-feature.md](issue-feature.md) | 100行を超える参照ファイルに、目次を足す要望 |
| `decision` | [decision.md](decision.md) | スキルを1つずつ単独のリポジトリにすると決めた記録 |

## サンプルの前提

- 題材は、このリポジトリを 2026-10-05 に公開したときの実際の作業と、公開されている資料である
- コマンドの出力とエラーメッセージは、2026-10-05 に Windows 11、Node.js 24.15.0、skills CLI 1.7.0 で実際に流した結果である
- 資料から引いた規則と数値は、2026-10-05 に読んだ版にもとづく。出典は各文書に書いている
- issue、PR、提案書、決定の記録は、文書の型を示すためのサンプルで、GitHub には投稿していない。文書どうしの参照は、このフォルダのファイルへのリンクで書いている
- 発火テストと、仕様の検査ツール `skills-ref` は流していない。流していないことは、関係する文書に書いている
