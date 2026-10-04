# 調査報告: document-writing を npx skills add で入れられるか

## 概要

`document-writing` は、`npx skills add` で入れられます。リポジトリのルートに `SKILL.md` を置くいまの構成のままで検出され、16個のファイルがコピーされました。
frontmatter を直す前の版は、検出されませんでした。利用者は、README にある1行のコマンドでスキルを入れられます。

## 背景

2026-10-05 に、`document-writing` を GitHub に公開しました。公開した時点の README は、`git clone` で入れる手順だけを案内していました。
skills CLI の `npx skills add` で入れられるか、入れるためにフォルダの構成を変える作業が要るかは、分かっていませんでした。

## 方法

skills CLI に、スキルの検出とインストールをさせ、出力とコピーされたファイルを確かめました。

| 項目 | 内容 |
| :-- | :-- |
| 環境 | Windows 11、Node.js 24.15.0、skills CLI 1.7.0 |
| 対象1 | 公開したリポジトリ `sumx21t-3310/document-writing` の `main` |
| 対象2 | frontmatter を直す前のコミット `b7ad31a` の `SKILL.md` を、手元のフォルダに取り出したもの |
| 検出のコマンド | `npx -y skills add <対象> --list` |
| インストールのコマンド | 空のフォルダで `npx -y skills add sumx21t-3310/document-writing -a claude-code -y` |
| ファイルの確かめ方 | インストールしたフォルダのファイルを一覧にして数える |

## 結果

| 対象 | 操作 | 結果 |
| :-- | :-- | :-- |
| 公開したリポジトリ | 検出 | `Found 1 skill`。`document-writing` と `description` が表示された |
| 公開したリポジトリ | インストール | `✓ document-writing (copied)`。`.claude/skills/document-writing` に16個のファイルができた |
| 直す前の `SKILL.md` | 検出 | `No skills found`。YAML を読めないという警告が出た |

コピーされた16個は、`SKILL.md`、`README.md`、`LICENSE`、`rules/common.md`、`locales/ja.md`、`reference/themes/` の YAML 11個です。インストールしたフォルダには、ほかに `skills-lock.json` ができました。

直す前の `SKILL.md` で出た警告は、次のとおりです。フォルダのパスは `<作業フォルダ>` に置き換えています。

```text
⚠ Skipped <作業フォルダ>\document-writing\SKILL.md — YAML parse error: Nested mappings are not allowed in compact mappings at line 2, column 14:
description: ドキュメントを設計してから執筆する。次のときに使う: 「手順書を書いて」「PRを作成して」「issueを書いて」「文章を執筆して」「…
             ^
◇  No skills found
│
└  No valid skills found. Skills require a SKILL.md with name and description.
```

## 考察

skills CLI は、frontmatter を YAML として読めた `SKILL.md` だけを、スキルとして扱うと考えられます。2つの対象の違いは、`description` をクォートで囲んだかどうかだけでした。
この結果を受けて、README の「インストール」に `npx skills add` の手順を足しました（コミット `18de589`）。フォルダの構成を変える作業は要りません。ルートに `SKILL.md` がある構成で、検出もインストールもできました。

## 実行しなかった項目

- インストールしたスキルを、エージェントが読み込んで動かすことの確認。CLI の動きを先に確かめたため
- Claude Code 以外のエージェントを選んだインストール。手元で使うエージェントを先に確かめたため
- 人が手で実行したときの、選択肢の画面の確認。確認の質問を省く `-y` を付けて実行したため
- [skills.sh](https://www.skills.sh/) の一覧に載るかどうかの確認。載る条件を調べていないため

## 次の作業

インストールしたスキルを、エージェントが読み込んで動かすことを確かめます。
