# GitHub Stats

[English](README.md) | [中文](README.zh.md) | 日本語

プロフィール README 用に毎日更新される GitHub 統計情報の画像を自動生成します。

このプロジェクトは [jstrieb](https://github.com/jstrieb) 氏による [github-stats](https://github.com/jstrieb/github-stats) を改良・強化したバージョンです。主要な改善点と違いについては、[オリジナルプロジェクトとの違い](#オリジナルプロジェクトとの違い)のセクションで詳しく説明しています。

![TeddyHuang-00 の GitHub 概覧（ダークモード）](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-dark-mode-only)
![TeddyHuang-00 の GitHub 概覧（ライトモード）](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-light-mode-only)
![TeddyHuang-00 のプログラミング言語統計（ダークモード）](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-dark-mode-only)
![TeddyHuang-00 のプログラミング言語統計（ライトモード）](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-light-mode-only)

## クイックスタート

1. **GitHub 個人アクセストークンの取得**
   1. [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens) に移動します。
   2. `repo`、`read:user`、および `read:org` のスコープを持つ新しいトークンを作成します。
   3. **ページを閉じた後は再度確認できないため、トークンをコピーして直ちに保存してください。**
2. **リポジトリの作成**
   1. このページの右上にある **"Use this template"** ボタンをクリックし、**"Create a new repository"** を選択します。
   2. リポジトリ名を入力します（例：`github-stats-rs`、または任意の名前）。
   3. **"Create repository"** をクリックして確定します。
3. **リポジトリシークレットの設定**
   1. 新規作成したリポジトリの **Settings > Secrets and variables > Actions** に移動します。
   2. **"New repository secret"** をクリックし、`ACCESS_TOKEN` という名前のシークレットを作成し、ステップ 1 でコピーした PAT を値として貼り付けます。
   3. **"Add secret"** をクリックして保存します。
   4. _(オプション)_ より詳細なカスタマイズのため、他のシークレットを追加します。詳細は[設定](#設定)セクションを参照してください。
4. _(オプション)_ **ワークフロー設定の調整**
   1. ワークフローファイル `.github/workflows/generate.yml` を編集して他の設定を変更できます：
      - ワークフローのスケジュールやコミットメッセージを変更。
      - 他のオプションを変更。詳細は[設定](#設定)セクションを参照してください。
5. **イメージの生成と実行**
   1. 設定が完了したら、ワークフローを手動で実行するか、スケジュール実行を待ちます。
      - 手動で実行するには、リポジトリの **Actions** タブに移動し、**"Generate Stats Images"** ワークフローを選択し、**"Run workflow"** ボタンをクリックします。
   2. ワークフローが完了すると、生成された画像はリポジトリの`generated`ブランチで利用可能になります。
6. **画像の埋め込み**
   [プロフィール README](https://docs.github.com/ja/account-and-profile/how-tos/profile-customization/managing-your-profile-readme) に以下の Markdown コードを追加します：

   ```markdown
   ![Username の GitHub 概覧（ダークモード）](https://raw.githubusercontent.com/Username/Repository/generated/overview.svg#gh-dark-mode-only)
   ![Username の GitHub 概覧（ライトモード）](https://raw.githubusercontent.com/Username/Repository/generated/overview.svg#gh-light-mode-only)
   ![Username のプログラミング言語統計（ダークモード）](https://raw.githubusercontent.com/Username/Repository/generated/languages.svg#gh-dark-mode-only)
   ![Username のプログラミング言語統計（ライトモード）](https://raw.githubusercontent.com/Username/Repository/generated/languages.svg#gh-light-mode-only)
   ```

   > `Username` と `Repository` を実際の GitHub ユーザー名とリポジトリ名に置き換えてください。
   >
   > ワークフローファイルでブランチ名を変更した場合は、URL もそれに応じて更新してください。

7. 自動更新される GitHub 統計情報を楽しんでください！

## 設定

オプションの設定は、リポジトリシークレットを追加するか、ワークフローファイルを直接編集することで設定できます：

> すべての設定はデフォルト値を使用するために削除可能です。
>
> ニーズに応じて、リポジトリシークレットを使用するか、ワークフローファイルに値をハードコードするかを選択できます。シークレットを使用する方がより安全ですが、ファイルを編集する方が便利です。**`ACCESS_TOKEN` などの機密情報を必ずシークレットとして使用してください。**

| 設定オプション    | 場所                   | 説明                                                                                                                           | デフォルト値 |
| :---------------- | :--------------------- | :----------------------------------------------------------------------------------------------------------------------------- | :----------- |
| `ACCESS_TOKEN`    | リポジトリシークレット | `repo`、`read:user`、`read:org` のスコープを持つ GitHub 個人アクセストークン(PAT)。                                            | 必須         |
| `EXCLUDE_REPOS`   | リポジトリシークレット | 統計から除外するリポジトリの完全名のカンマ区切りリスト。形式：`username/repository`。                                          | 空           |
| `EXCLUDE_LANGS`   | リポジトリシークレット | 統計から除外するプログラミング言語のカンマ区切りリスト。表示名を使用（例：`Jupyter Notebook`）。大文字小文字は区別されません。 | 空           |
| `EXCLUDE_FORKS`   | ワークフローファイル   | `true` の場合、フォークされたリポジトリは統計から除外されます。                                                                | `false`      |
| `EXCLUDE_PRIVATE` | ワークフローファイル   | `true` の場合、プライベートリポジトリは統計から除外されます。                                                                  | `false`      |
| `EXCLUDE_ARCHIVE` | ワークフローファイル   | `true` の場合、アーカイブされたリポジトリは統計から除外されます。                                                              | `false`      |
| `LOG_LEVEL`       | ワークフローファイル   | ログレベル: `0` (オフ), `1` (エラー), `2` (警告), `3` (情報), `4` (デバッグ), `5` (トレース)。                                 | `2`          |
| `DELAY_MS`        | ワークフローファイル   | API レート制限に達しないようにするため、API リクエスト間の遅延（ミリ秒単位）。                                                 | `1000`       |
| `OUTPUT_BRANCH`   | ワークフローファイル   | 生成された画像が保存されるブランチ名。                                                                                         | `generated`  |

## オリジナルプロジェクトとの違い

このプロジェクトは、オリジナルの [github-stats](https://github.com/jstrieb/github-stats) を Rust ベースで再構築したものです。主な改善点は以下の通りです：

- **コード行数の合計を削除**: GitHub の API はリポジトリやコミットの変更行数情報を提供しないため、[オリジナル作者が指摘](https://github.com/jstrieb/github-stats#disclaimer)しているように、この指標はあまり正確ではなかったため、削除されました。
- **フォロワー数の追加**: プロフィール統計にフォロワー総数が含まれるようになり、GitHub 上でのユーザーの影響力をより適切に反映します。
- **大幅なパフォーマンス向上**: API レート制限を回避するためにリクエストのスロットリングとシングルスレッド処理を実装していますが、生成ははるかに高速です。（テストでは、約 100 のリポジトリを持つユーザーの場合、オリジナルのワークフローは約 24 分かかり、時折失敗していましたが、この Rust バージョンは約 5 分で確実に完了します。）
- **最新の GitHub API クライアント**: このプロジェクトでは、Rust 用の最新かつメンテナンスが行き届いた GitHub API クライアントである `octocrab` クレートを使用しており、GitHub API との長期的な互換性と信頼性を確保しています。
- **より多くの設定オプション**: プライベートリポジトリを統計から除外するなど、より多くのカスタマイズオプションが追加されました。
- **常に最新の依存関係**: [Renovate](https://github.com/renovatebot/renovate) を使用して、すべての依存関係が常に最新の状態に保たれ、API 互換性と最新の修正・機能へのアクセスが保証されます。
- **コード品質と保守性の向上**: コードベースは、可読性、保守性、パフォーマンスに重点を置いて Rust で再作成されました。`octocrab` と REST API を使用することで、オリジナルの GraphQL 実装と比較してコードがよりクリーンでシンプルになっています。
- **クリーンなリポジトリ構造**: ワークフローは生成された画像を別のブランチ（デフォルトは `generated`）に出力し、メインブランチはソースコードに焦点を当て、クリーンな履歴を維持します。これにより、生成ファイルからの競合なしにこのテンプレートからの更新を簡単に同期できます。

## 注意事項

- **ログと機密情報**: ログレベルが `0`（オフ）以外、特に `4`（デバッグ）以上の設定の場合、ワークフローログに機密情報（例：プライベートリポジトリ名、トークンなど）が含まれる可能性があります。ログを公開で共有する際は注意してください。これらのログは GitHub によって限定的な期間（90 日）のみ保持され、ユーザーと GitHub 管理者にのみ表示されます。
- **予期せぬエラーの可能性**: 稀に、コードエラーにより意図せず機密データが出力される場合があります（例：API が予期しない応答を返し、パースライブラリがパニックを引き起こす場合）。このような問題が発生した場合は、速やかに報告してください。
- **統計の正確性**: GitHub API の制限やリクエスト間のデータ変更により、統計に不正確さが生じる可能性があります。これを最小限に抑えるための努力が行われていますが、完全に排除することはできません。統計の信頼性を評価するために、定期的に Actions ログの警告やエラーを確認することをお勧めします。

## 開発

ローカルでプロジェクトを実行するには、GitHub 個人アクセストークンなどの設定をプロジェクトルートに `.env` ファイルとして保存できます。参考として [`.env.sample`](.env.sample) ファイルを参照してください。

このプロジェクトでは、これらの設定を使用してプロジェクトを実行するための `justfile` を提供しています。[Just](https://just.systems/) がインストールされていることを確認してください。その後、以下のコマンドを使用できます：

```bash
# コードのフォーマットとリンティングの問題を修正
just fix
# プロジェクトの実行
just run
```

## 貢献

貢献は大歓迎です！問題を発見したり、改善の提案がある場合は、遠慮なく issue を作成するか、pull request を送信してください。

## ライセンス

このプロジェクトではオリジナルプロジェクトの SVG テンプレートのみを使用しています。すべてのコードは完全に異なるアプローチで Rust で書き直されています。それにもかかわらず、これはオリジナルの派生作品と見なされます。したがって、オリジナルプロジェクトのライセンスに従い、このコードも GPLv3 ライセンスの下でライセンスされています。詳細については [LICENSE](LICENSE) ファイルを参照してください。

---

**注記**: 本ドキュメントは LLM（大規模言語モデル）により自動翻訳を生成し、重要な情報の正確性を確保するため簡易的な校正のみを実施しています。文法や表現の不自然さについては予めご了承ください。
