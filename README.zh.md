# GitHub 统计图表

[English](README.md) | 中文 | [日本語](README.ja.md)

为你的个人主页 README 生成每日自动更新的 GitHub 统计图表。

本项目基于 [jstrieb](https://github.com/jstrieb) 的 [github-stats](https://github.com/jstrieb/github-stats) 修改优化。主要改进和不同之处请见[与原项目的差异](#与原项目的差异)部分。

![TeddyHuang-00 的 GitHub 概览 (深色模式)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-dark-mode-only)
![TeddyHuang-00 的 GitHub 概览 (浅色模式)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-light-mode-only)
![TeddyHuang-00 的编程语言统计 (深色模式)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-dark-mode-only)
![TeddyHuang-00 的编程语言统计 (浅色模式)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-light-mode-only)

## 快速开始

1. **获取 GitHub 个人访问令牌**
   1. 前往 [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens)。
   2. 创建一个新的令牌，并勾选 `repo`、`read:user` 和 `read:org` 权限。
   3. **创建后请立即复制保存，页面关闭后将无法再次查看。**
2. **创建你的仓库**
   1. 点击本页面右上角的 **"Use this template"** 按钮，然后选择 **"Create a new repository"**。
   2. 填写仓库名称（例如 `github-stats-rs`，或任何你喜欢的名称）。
   3. 点击 **"Create repository"** 确认。
3. **配置仓库密钥**
   1. 前往你新创建仓库的 **Settings > Secrets and variables > Actions**。
   2. 点击 **"New repository secret"**，添加一个名为 `ACCESS_TOKEN` 的密钥，并将第一步复制的 PAT 粘贴为值。
   3. 点击 **"Add secret"** 保存。
   4. _(可选)_ 如需更多自定义，可参考[配置](#配置)部分添加其他密钥。
4. _(可选)_ **调整工作流设置**
   1. 你可以编辑 `.github/workflows/generate.yml` 工作流文件来配置其他设置：
      - 更改工作流的运行计划时间，或修改提交信息。
      - 也可以修改其他选项。详情请参阅[配置](#配置)部分。
5. **运行并生成图表**
   1. 配置完成后，你可以手动触发工作流或等待其按计划时间自动运行。
      - 要手动触发，请前往仓库的 **Actions** 标签页，选择 **"Generate Stats Images"** 工作流，然后点击 **"Run workflow"** 按钮。
   2. 工作流完成后，生成的图片将出现在你仓库的 `generated` 分支中。
6. **嵌入图表**
   将以下 Markdown 代码添加到你的个人主页 README 中：

   ```markdown
   ![用户名 的 GitHub 概览 (深色模式)](https://raw.githubusercontent.com/用户名/仓库名/generated/overview.svg#gh-dark-mode-only)
   ![用户名 的 GitHub 概览 (浅色模式)](https://raw.githubusercontent.com/用户名/仓库名/generated/overview.svg#gh-light-mode-only)
   ![用户名 的编程语言统计 (深色模式)](https://raw.githubusercontent.com/用户名/仓库名/generated/languages.svg#gh-dark-mode-only)
   ![用户名 的编程语言统计 (浅色模式)](https://raw.githubusercontent.com/用户名/仓库名/generated/languages.svg#gh-light-mode-only)
   ```

   > 将 `用户名` 和 `仓库名` 替换为你实际的 GitHub 用户名和仓库名称。
   >
   > 如果你在工作流文件中更改了分支名称，请确保相应地更新 URL。

7. 享受你的动态 GitHub 统计图片吧！

## 配置

以下为可选配置，可通过添加仓库密钥或直接修改工作流文件来设置：

> 所有配置均可移除，将使用其默认值。
>
> 你可以根据需求，通过修改工作流文件来引用仓库密钥，或将某些配置直接写入工作流文件。密钥方式更安全，直接编辑则更方便。对于敏感信息（**尤其是 `ACCESS_TOKEN`**），务必使用密钥。

| 配置项            | 位置       | 说明                                                                                                  | 默认值      |
| ----------------- | ---------- | ----------------------------------------------------------------------------------------------------- | ----------- |
| `ACCESS_TOKEN`    | 仓库密钥   | 具有 `repo`、`read:user` 和 `read:org` 权限的 GitHub 个人访问令牌（PAT）。                            | 必填        |
| `EXCLUDE_REPOS`   | 仓库密钥   | 要从统计中排除的仓库名称列表，以英文逗号分隔。应为完整名称，即 `用户名/仓库名`。                      | 空          |
| `EXCLUDE_LANGS`   | 仓库密钥   | 要从统计中排除的编程语言列表，以英文逗号分隔。应使用显示名称，例如 `Jupyter Notebook`。不区分大小写。 | 空          |
| `EXCLUDE_FORKS`   | 工作流文件 | 如果为 `true`，则分叉（Fork）的仓库将被排除在统计之外。                                               | `false`     |
| `EXCLUDE_PRIVATE` | 工作流文件 | 如果为 `true`，则私有仓库将被排除在统计之外。                                                         | `false`     |
| `EXCLUDE_ARCHIVE` | 工作流文件 | 如果为 `true`，则已归档的仓库将被排除在统计之外。                                                     | `false`     |
| `LOG_LEVEL`       | 工作流文件 | 日志级别：`0`（关闭）、`1`（错误）、`2`（警告）、`3`（信息）、`4`（调试）、`5`（追踪）。              | `2`         |
| `DELAY_MS`        | 工作流文件 | API 请求间隔（毫秒），用于防止触发速率限制。                                                          | `1000`      |
| `OUTPUT_BRANCH`   | 工作流文件 | 存放生成图表的分支名称。                                                                              | `generated` |

## 与原项目的差异

本项目是原 [github-stats](https://github.com/jstrieb/github-stats) 的 Rust 重构版本，主要改进如下：

- **移除代码总行数统计**：由于 GitHub API 未提供仓库或提交的代码变更行数信息，且[原作者也指出](https://github.com/jstrieb/github-stats#disclaimer)该指标原本就不够精确，故予以移除。
- **新增关注者数量**：在概览统计中增加了关注者总数，以更好地体现用户在 GitHub 上的影响力。
- **性能大幅提升**：尽管加入了请求间隔且为单线程处理以避免触及 API 速率限制，生成速度依然显著加快。（实测显示，对于一个拥有约 100 个仓库的用户，原工作流需约 24 分钟且偶有失败，而本 Rust 版本仅需约 5 分钟且运行稳定。）
- **现代化的 GitHub API 客户端**：核心使用了 `octocrab` 库，这是一个维护活跃、设计现代的 GitHub API 客户端，为项目的长期可靠性提供了保障，确保与 GitHub API 的兼容性。
- **更丰富的配置选项**：增加了如排除私有仓库等更多自定义选项。
- **依赖持续更新**：通过 [Renovate](https://github.com/renovatebot/renovate) 保持所有依赖为最新版本，确保 API 兼容性及获取修复和更新。
- **代码质量与可维护性提升**：使用 Rust 重写后的代码库更注重可读性、可维护性和性能。通过使用 `octocrab` 和 REST API，代码逻辑相比原项目的 GraphQL 实现更为简洁清晰。
- **更清晰的仓库结构**：工作流默认将生成的图表输出到独立的 `generated` 分支，使主分支仅包含项目源码，历史记录更加整洁。你可以轻松地同步本模板的更新，而无需担心生成的图表文件造成冲突或干扰。

## 注意事项

- **日志与敏感信息**：如果将日志级别设置为 `0` (关闭) 以外的值，尤其是 `4` (调试) 或更高，工作流日志中可能会记录一些敏感信息（如私有仓库名、令牌等）。请谨慎公开分享日志。这些日志仅由 GitHub 保留一段时间（90 天），且仅对您和 GitHub 管理员可见。
- **潜在的意外错误**：在极少数情况下，代码可能因意外错误而输出敏感数据（例如 API 返回异常响应导致解析库报错）。如果您发现此类情况，请及时提交 Issue。
- **统计数据的准确性**：由于 GitHub API 的限制或请求间数据变动，统计结果可能出现偏差。我们已尽力减少此类问题，但无法完全避免。建议定期查看 Actions 日志中的警告或错误信息，以评估统计数据的可靠性。

## 本地开发

若要在本地运行项目，可以将 GitHub 个人访问令牌等配置信息存放在项目根目录的 `.env` 文件中。具体格式请参考 [`.env.sample`](.env.sample) 示例文件。

本项目提供了一个 `justfile` 使用这些配置来运行项目。请确保已安装 [Just](https://just.systems/) 命令运行器，然后即可使用如下命令：

```bash
# 修复代码格式和规范问题
just fix
# 运行项目
just run
```

## 贡献

我们欢迎任何形式的贡献！如果你发现了问题或有改进的想法，请随时提交 Issue 或 Pull Request。

## 许可证

本项目仅使用了原项目的 SVG 模板，全部代码均采用 Rust 重写，且实现思路完全不同。尽管如此，我们仍视其为原项目的衍生作品。因此，依照原项目的许可，本项目代码同样采用 GPLv3 许可证。详情请见 [LICENSE](LICENSE) 文件。
