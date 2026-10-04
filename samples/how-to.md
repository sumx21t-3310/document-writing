# SKILL.md の frontmatter を、YAML として読める形に直す

`SKILL.md` の frontmatter を YAML として読めないと、skills CLI はスキルを検出しません。
この手順では、`description` をクォートで囲み、frontmatter を YAML として読める形に直します。

## この手順でできること

手順を終えると、frontmatter の検査が通り、`npx skills add` がスキルを検出します。

## 前提条件

- [uv](https://docs.astral.sh/uv/) と Node.js がインストールされていること。この手順は Node.js 24.15.0 と skills CLI 1.7.0 で確かめている
- `SKILL.md` の frontmatter に、`: `（コロンと空白）を含む値が、クォートなしで書かれていること
- `SKILL.md` を編集できること

この手順では、次の frontmatter を例にします。`description` の値に `次のときに使う: ` と `Do NOT use for: ` が含まれています。

```markdown
---
name: document-writing
description: ドキュメントを設計してから執筆する。次のときに使う: 「手順書を書いて」「PRを作成して」「issueを書いて」「文章を執筆して」「作業報告をまとめて」「提案書を書いて」「この文書を添削して」と頼まれた、README・設計メモ・仕様・調査報告・紹介記事を書く、既存の文書を書き直す・添削する。Do NOT use for: コミットメッセージ、コード内コメント、UI 文言、3 行以内の連絡。エラーメッセージ・スタックトレース・取り消せない操作の確認は要約せず全文を示す対象のため、このスキルで書き直さない。
---
```

## 手順1: frontmatter を検査する

`SKILL.md` があるフォルダで、次のコマンドを実行します。frontmatter を YAML として読み、項目の名前を表示するコマンドです。

```bash
uv run -q --with pyyaml python -c "import sys,yaml; print(list(yaml.safe_load(open(sys.argv[1],encoding='utf-8').read().split('---')[1])))" SKILL.md
```

読めない frontmatter では、出力の最後が次のエラーになります。

```text
yaml.scanner.ScannerError: mapping values are not allowed here
  in "<unicode string>", line 3, column 39:
     ... ption: ドキュメントを設計してから執筆する。次のときに使う: 「手順書を書いて」「PRを作成して」「issueを書いて」「 ... 
                                         ^
```

## 手順2: エラーから、直す行を読み取る

エラーの `line` の数字が、`SKILL.md` の行番号です。例では3行目の `description` です。
`^` は、値の中にある `: ` の位置を指しています。

## 手順3: 値の全体を、シングルクォートで囲む

手順2で見つけた行の値を、先頭から末尾までシングルクォートで囲みます。値の文面は変えません。

```markdown
---
name: document-writing
description: 'ドキュメントを設計してから執筆する。次のときに使う: 「手順書を書いて」「PRを作成して」「issueを書いて」「文章を執筆して」「作業報告をまとめて」「提案書を書いて」「この文書を添削して」と頼まれた、README・設計メモ・仕様・調査報告・紹介記事を書く、既存の文書を書き直す・添削する。Do NOT use for: コミットメッセージ、コード内コメント、UI 文言、3 行以内の連絡。エラーメッセージ・スタックトレース・取り消せない操作の確認は要約せず全文を示す対象のため、このスキルで書き直さない。'
---
```

## 手順4: もう一度検査する

手順1と同じコマンドを実行します。項目の名前が表示されれば、YAML として読めています。

```text
['name', 'description']
```

## 結果を確かめる

スキルのフォルダの1つ上のフォルダで、次のコマンドを実行します。`<フォルダ名>` は、スキルのフォルダの名前に置き換えます。

```bash
npx -y skills add ./<フォルダ名> --list
```

出力に次の行が含まれていれば終わりです。

```text
◇  Found 1 skill
```

## うまくいかないとき

手順4で同じエラーが別の行に出たときは、その行の値にも `: ` が含まれています。その行について、手順2からもう一度進めてください。

値の中にシングルクォートが含まれているときは、手順3のあとで別のエラーが出ます。YAML では、シングルクォートで囲んだ値の中のシングルクォートを `''` と2つ重ねて書きます。この場合の動きは、この手順では確かめていません。
