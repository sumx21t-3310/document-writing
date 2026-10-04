# description をクォートで囲み、frontmatter を YAML として読める形にする

## 概要

`SKILL.md` の `description` を、シングルクォートで囲みました。
frontmatter を YAML として読めるようにして、skills CLI がこのスキルを検出できるようにする変更です。`description` の文面は変えていません。

## 背景

`description` の値に `次のときに使う: ` と `Do NOT use for: ` が含まれ、クォートなしで書かれていました。
YAML では、値の中の `: ` が項目の区切りとして読まれます。そのため、frontmatter の読み込みがエラーになり、`npx skills add` はこのスキルを検出しませんでした。経緯は [issue-bug.md](issue-bug.md) にあります。

## 変更内容

変更したのは `SKILL.md` の3行目だけです。`description` の値の前後に、シングルクォートを1つずつ足しました。

## 確認方法

次の2つを、変更の前後で実行しました。

| 確認 | 変更前 | 変更後 |
| :-- | :-- | :-- |
| PyYAML で frontmatter を読む | `ScannerError: mapping values are not allowed here` | `['name', 'description']` |
| `npx -y skills add <対象> --list` | `No skills found` | `Found 1 skill` |

実行したコマンドの全文は、[how-to.md](how-to.md) の手順1にあります。
このリポジトリには、自動のテストがありません。上の2つは手で実行しました。

## 影響範囲

スキルの本文、ルール、テーマファイルには触れていません。
Claude Code は、変更前の `SKILL.md` も読み込めていました。Claude Code での呼ばれ方が変わるかどうかは、確かめていません。

## レビューで見てほしい点

囲み方に、シングルクォートを選びました。値の中に、シングルクォートが含まれていないためです。

長い値を複数の行に分ける書き方（`>-`）は、試していません。`description` は、長い1行のままです。行を分ける書き方にそろえるほうがよければ、指摘してください。
