# SKILL.md の frontmatter の項目と、分量の目安

`SKILL.md` の frontmatter に書ける項目、スキルのフォルダの構成、分量の目安の一覧です。
値は、2026-10-05 に読んだ [Agent Skills の仕様](https://agentskills.io/specification) から書いています。Anthropic の資料だけにある制約は、節を分けています。

## frontmatter の項目

| 項目 | 必須 | 制約 | 説明 | 例 |
| :-- | :-- | :-- | :-- | :-- |
| `name` | 必須 | 1〜64文字。小文字の英字、数字、ハイフンだけ。先頭と末尾はハイフン以外。ハイフンを2つ続けない。フォルダ名と同じ文字列 | スキルの名前 | `pdf-processing` |
| `description` | 必須 | 1〜1024文字 | スキルがすることと、使う場面 | `PDF からテキストを取り出す。PDF を扱うときに使う。` |
| `license` | 任意 | 制約なし。短く書く | ライセンスの名前、または同梱したライセンスファイルの名前 | `Apache-2.0` |
| `compatibility` | 任意 | 1〜500文字 | 動作に要る環境。対象の製品、必要なパッケージ、ネットワークの要否 | `Requires git and jq` |
| `metadata` | 任意 | キーと値がどちらも文字列の対応表 | 仕様にない項目を置く場所 | `author: example-org` |
| `allowed-tools` | 任意 | 空白で区切った文字列。実験的な項目 | 確認なしで使えるツール | `Bash(git:*) Read` |

## フォルダの構成

| 名前 | 必須 | 制約 | 説明 | 例 |
| :-- | :-- | :-- | :-- | :-- |
| `SKILL.md` | 必須 | YAML の frontmatter のあとに、Markdown の本文を書く | スキルの本体 | `pdf-processing/SKILL.md` |
| `scripts/` | 任意 | 制約なし | エージェントが実行するコード | `scripts/extract.py` |
| `references/` | 任意 | 制約なし | エージェントが要るときに読む資料 | `references/REFERENCE.md` |
| `assets/` | 任意 | 制約なし | テンプレート、画像、データ | `assets/template.md` |

## 分量の目安

| 対象 | 読み込まれる時点 | 目安 | 説明 | 例 |
| :-- | :-- | :-- | :-- | :-- |
| `name` と `description` | エージェントの起動時。すべてのスキルの分を読む | 約100トークン | 呼ぶかどうかの判断に使う | frontmatter の2行 |
| `SKILL.md` の本文 | スキルが呼ばれた時 | 5000トークン未満、500行未満 | 本文の全体を読む | 手順、例 |
| ほかのファイル | 手順の中で要る時 | 目安なし | 要るファイルだけを読む | `references/` の資料 |

## Anthropic の資料にある制約

次の制約は、[Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices) にあり、仕様にはありません。

| 項目 | 必須 | 制約 | 説明 | 例 |
| :-- | :-- | :-- | :-- | :-- |
| `name` | 必須 | XML のタグを含めない。`anthropic` と `claude` の語を含めない | 予約された語を避ける | `claude-tools` は使えない |
| `description` | 必須 | XML のタグを含めない | 仕様と同じ1024文字の上限もある | — |
| 参照ファイル | 任意 | 100行を超えるファイルは、先頭に目次を置く | 途中までしか読まれなくても、全体の範囲が分かる | `## Contents` |

## 共通の決まり

| 項目 | 決まり | 説明 | 例 |
| :-- | :-- | :-- | :-- |
| ファイルの参照 | スキルのルートからの相対パスで書く | 絶対パスは使わない | `references/REFERENCE.md` |
| 参照の深さ | `SKILL.md` から1段までにする | 参照先のファイルから、さらに別のファイルを参照しない | `SKILL.md` → `references/a.md` |
| パスの区切り | スラッシュを使う | Anthropic の資料にある決まり | `scripts/helper.py` |
| 検査 | `skills-ref validate <フォルダ>` | frontmatter と名前の決まりを検査する。仕様が案内しているツール | `skills-ref validate ./my-skill` |
