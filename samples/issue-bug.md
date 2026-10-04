# SKILL.md の frontmatter を YAML として読めず、npx skills add でスキルが見つからない

## 概要

コミット `b7ad31a` の `SKILL.md` を `npx skills add` に渡すと、スキルが検出されません。
frontmatter の `description` を、YAML として読めないためです。

## 期待した動き

`npx skills add` が `Found 1 skill` と表示し、`document-writing` を一覧に出すこと。

## 実際の動き

次の警告が出て、`No skills found` で終わります。フォルダのパスは `<作業フォルダ>` に置き換えています。

```text
⚠ Skipped <作業フォルダ>\document-writing\SKILL.md — YAML parse error: Nested mappings are not allowed in compact mappings at line 2, column 14:
description: ドキュメントを設計してから執筆する。次のときに使う: 「手順書を書いて」「PRを作成して」「issueを書いて」「文章を執筆して」「…
             ^
◇  No skills found
│
└  No valid skills found. Skills require a SKILL.md with name and description.
```

## 再現手順

1. このリポジトリを clone し、リポジトリのフォルダへ移動する
2. 空の作業フォルダを作り、その中に `document-writing` フォルダを作る
3. `git show b7ad31a:SKILL.md` の出力を、手順2の `document-writing` フォルダに `SKILL.md` として保存する
4. 作業フォルダへ移動し、`npx -y skills add ./document-writing --list` を実行する

2回試して、2回とも同じ動きになりました。

## 環境

| 項目 | 値 |
| :-- | :-- |
| document-writing | コミット `b7ad31a` |
| skills CLI | 1.7.0 |
| Node.js | 24.15.0 |
| OS | Windows 11 |

## 影響範囲と回避策

`npx skills add` でこのスキルを入れようとした人の全員が、スキルを入れられません。
Claude Code は、同じ `SKILL.md` を読み込めていました。`git clone` でスキルのフォルダに入れた Claude Code の利用者には、影響がありません。Claude Code 以外のエージェントで読めるかどうかは、確かめていません。

回避策は、`git clone` で入れることです。

## 原因

PyYAML で同じ frontmatter を読むと、次のエラーになります。

```text
yaml.scanner.ScannerError: mapping values are not allowed here
  in "<unicode string>", line 3, column 39:
     ... ption: ドキュメントを設計してから執筆する。次のときに使う: 「手順書を書いて」「PRを作成して」「issueを書いて」「 ... 
                                         ^
```

`^` が指しているのは、`次のときに使う: ` の `:` です。

ここからは推測です。`description` の値がクォートで囲まれていないので、値の中の `: ` が、項目の区切りとして読まれたと考えられます。直し方は [how-to.md](how-to.md) にあります。
