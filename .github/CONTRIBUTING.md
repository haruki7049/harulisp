# CONTRIBUTING GUIDE

This Guide is written in Japanese for Japanese Developer. If you want the English version, please post on [GitHub Discussions](https://github.com/haruki7049/hrtor/discussions). このガイドは日本人の開発者用に日本語で書かれています。英語版が欲しい場合は[GitHub Discussions](https://github.com/haruki7049/hrtor/discussions)に投稿してください。

`CONTRIBUTING.md`に対して質問や疑問が生じた場合には[GitHub Discussions](https://github.com/haruki7049/hrtor/discussions)に投稿してください。

## Issueの作成

- Issueを立てる前には必ず同様のIssueが存在しない事を確認してください。もしもIssueの内容が被っていた場合には`Not Planned`としてIssueを閉じてください。
- Issueをグループにしたい場合はマイルストーンを作成する事を検討してください。

### Issueの命名規則

- Issueの命名規則は[Conventional Commits:v1.0.0](https://www.conventionalcommits.org/ja/v1.0.0/)に従ってください。

以下に例を提示します。

`feat: Adding regex_lite in Cargo.toml`, `fix: Fixing typo`, `refactor: Deleting hoge function`, `docs: Writing CONTRIBUTING.md`

## ブランチの命名規則

ブランチの命名は以下の様に行なってください。

```txt
(Issueの型)/(Issueの説明文を全て小文字で表現、程良く短くしてハイフン区切りにした文)
```

以下に例を提示します。

`feat/adding-regex_lite`, `fix/fixing-typo`, `refactor/deleting-hoge-function`, `docs/writing-contributing`

## PullRequestの作成

PullRequestは必ずIssueに関連づけて作成してください。関連情報として[GitHub Docs](https://docs.github.com/ja/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue)を参照してください。

### PullRequestの命名規則

関連づけたIssueの名前と一致させてください。

## マイルストーンの作成

**現在マイルストーンに関するルールは制定されていません。議論は[ここ](https://github.com/haruki7049/harulisp/discussions/14)で行なってください。**

## コードの修正

以下のフローに従ってください。

1. Issueを作成する。
2. Issueに対応させたPullRequestを作成する。作成方法は[GitHub Docs](https://docs.github.com/ja/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue)
3. PullRequestにレビューを貰い、マージ。

## CI/CDの追加

現在、GitHub Actionsはrust-checkerのみ動作しています。以下にrust-checkerの説明を記載します。

- `cargo-test`の実行。
- `cargo-clippy --check`の実行。
- `cargo-fmt`によって適切にインデント等されているかの確認。

もしも他にCI/CDを使うべき作業がある場合はその旨を書いたIssueを作成してください。

## Review

レビューを貰いマージが出来るようになり、作業を終えた後には作業ブランチを削除してください。
