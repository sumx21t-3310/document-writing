# 最初のスキルを作り、プロジェクトに入れる

このチュートリアルでは、リリースノートを書くスキル `release-note` を作り、プロジェクトに入れます。
スキルを初めて作る人が対象です。ターミナルでコマンドを実行できれば、最後まで進められます。

## このチュートリアルで作るもの

完成すると、プロジェクトのフォルダに次のファイルができます。Claude Code は、このフォルダにあるスキルを読み込みます。

```text
.claude/skills/release-note/SKILL.md
```

作りながら、次の3つを覚えます。

- スキルのフォルダと `SKILL.md` を作る
- `SKILL.md` の先頭に、名前と説明を書く
- skills CLI で、スキルを検出してプロジェクトに入れる

## 準備

[Node.js](https://nodejs.org/) をインストールし、ターミナルで `npx` を使える状態にします。
この手順は、Node.js 24.15.0 と skills CLI 1.7.0 で確かめています。

## 手順1: スキルのフォルダと SKILL.md を作る

作業用のフォルダ `work` を作り、その中に `release-note` フォルダを作ります。
`release-note` フォルダの中に `SKILL.md` を作り、次の内容を書いて保存します。

```markdown
---
name: release-note
description: '変更の一覧からリリースノートを書く。リリースノート、変更履歴、CHANGELOG を頼まれたときに使う。'
---
# リリースノートを書く

1. 変更の一覧を、追加・変更・修正の3つに分ける
2. 利用者に影響する変更を先に並べる
3. 1つの変更を1行で書く
```

`---` で囲んだ先頭の部分を frontmatter と呼びます。`name` はフォルダ名と同じ文字列にします。`description` には、スキルがすることと、使う場面を書きます。

フォルダの中は、次の形になっています。

```text
work/
└── release-note/
    └── SKILL.md
```

## 手順2: スキルが検出されることを確かめる

ターミナルで `work` フォルダへ移動し、次のコマンドを実行します。`--list` を付けると、検出の結果を表示するだけで、どこにも入れません。

```bash
npx -y skills add ./release-note --list
```

出力に次の行が含まれていれば、スキルとして検出されています。

```text
◇  Found 1 skill
│
◇  Available Skills
│
│    release-note
│
│      変更の一覧からリリースノートを書く。リリースノート、変更履歴、CHANGELOG を頼まれたときに使う。
```

## 手順3: プロジェクトに入れる

スキルを使うプロジェクトのフォルダを用意します。ここでは、`work` の隣に空のフォルダ `proj` を作ります。
ターミナルで `proj` フォルダへ移動し、次のコマンドを実行します。

```bash
npx -y skills add ../work/release-note -a claude-code -y
```

`-a claude-code` で、入れる先のエージェントに Claude Code を選びます。最後の `-y` で、確認の質問を省きます。
出力に次の2行が含まれていれば、スキルが入っています。

```text
│  ✓ release-note (copied)                                                     │
└  Done!  Review skills before use; they run with full agent permissions.
```

## 完成したものを確かめる

`proj` フォルダの中を見ます。次の2つのファイルができていれば完成です。

```text
.claude/skills/release-note/SKILL.md
skills-lock.json
```

`.claude/skills/release-note/SKILL.md` を開くと、手順1で書いた内容がそのまま入っています。

このチュートリアルでは、エージェントがこのスキルを実際に呼ぶところまでは確かめていません。

## 次に読むもの

- [SKILL.md の frontmatter の項目と、分量の目安](reference.md): `name` と `description` に書ける文字と長さ
- [description が、スキルが呼ばれるかどうかを決める理由](explanation.md): 手順1の `description` の役割
- [SKILL.md の frontmatter を、YAML として読める形に直す](how-to.md): 手順2でスキルが検出されなかったときの直し方
