# GitHub Stats

English | [中文](README.zh.md) | [日本語](README.ja.md)

Generate automatically updated GitHub stats images for your profile README, updated daily.

This project is a modified and enhanced version of [github-stats](https://github.com/jstrieb/github-stats) by [jstrieb](https://github.com/jstrieb). Key improvements and differences are detailed in the [Differences from the Original Project](#differences-from-the-original-project) section.

![GitHub Overview for TeddyHuang-00 (Dark Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-dark-mode-only)
![GitHub Overview for TeddyHuang-00 (Light Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-light-mode-only)
![Languages Stats for TeddyHuang-00 (Dark Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-dark-mode-only)
![Languages Stats for TeddyHuang-00 (Light Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-light-mode-only)

## Quick Start

1. **Obtain GitHub Personal Access Token**
   1. Go to [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens).
   2. Create a new token with `repo`, `read:user`, and `read:org` scopes.
   3. **Copy and save it immediately, as you won't be able to see it again after closing the page.**
2. **Create Your Repository**
   1. Click the **"Use this template"** button at the top right of this page, then select **"Create a new repository"**.
   2. Enter a repository name (e.g., `github-stats-rs`, or any name you prefer).
   3. Click **"Create repository"** to confirm.
3. **Configure Repository Secrets**
   1. Go to your new repository's **Settings > Secrets and variables > Actions**.
   2. Click **"New repository secret"**, create a secret named `ACCESS_TOKEN`, and paste the PAT you copied in step 1 as its value.
   3. Click **"Add secret"** to save.
   4. _(Optional)_ Add other secrets for more customization. See the [Configuration](#configuration) section for details.
4. _(Optional)_ **Adjust Workflow Settings**
   1. You can edit the workflow file `.github/workflows/generate.yml` to configure other settings:
      - Change the workflow schedule or commit messages.
      - Modify other options. See the [Configuration](#configuration) section for details.
5. **Run and Generate Images**
   1. Once configured, you can manually trigger the workflow or wait for its scheduled run.
      - To trigger manually, go to your repository's **Actions** tab, select the **"Generate Stats Images"** workflow, and click the **"Run workflow"** button.
   2. After the workflow completes, the generated images will be available in the `generated` branch of your repository.
6. **Embed the Images**
   Add the following Markdown code to your [profile README](https://docs.github.com/en/account-and-profile/how-tos/profile-customization/managing-your-profile-readme):

   ```markdown
   ![GitHub Overview for Username (Dark Mode)](https://raw.githubusercontent.com/Username/Repository/generated/overview.svg#gh-dark-mode-only)
   ![GitHub Overview for Username (Light Mode)](https://raw.githubusercontent.com/Username/Repository/generated/overview.svg#gh-light-mode-only)
   ![Languages Stats for Username (Dark Mode)](https://raw.githubusercontent.com/Username/Repository/generated/languages.svg#gh-dark-mode-only)
   ![Languages Stats for Username (Light Mode)](https://raw.githubusercontent.com/Username/Repository/generated/languages.svg#gh-light-mode-only)
   ```

   > Replace `Username` and `Repository` with your actual GitHub username and repository name.
   >
   > If you changed the branch name in the workflow file, make sure to update the URLs accordingly.

7. Enjoy your dynamically updated GitHub stats!

## Configuration

Optional configurations can be set by adding repository secrets or editing the workflow file directly:

> All configurations can be removed to use their default values.
>
> You can choose to use repository secrets or hardcode values in the workflow file based on your needs. Using secrets is more secure, while editing the file is more convenient. **Always use a secret for sensitive information like `ACCESS_TOKEN`.**

| Config Option     | Location          | Description                                                                                                                         | Default Value |
| :---------------- | :---------------- | :---------------------------------------------------------------------------------------------------------------------------------- | :------------ |
| `ACCESS_TOKEN`    | Repository Secret | Your GitHub Personal Access Token (PAT) with `repo`, `read:user`, and `read:org` scopes.                                            | Required      |
| `EXCLUDE_REPOS`   | Repository Secret | Comma-separated list of repository full names to exclude from stats. Format: `username/repository`.                                 | Empty         |
| `EXCLUDE_LANGS`   | Repository Secret | Comma-separated list of programming languages to exclude from stats. Use display names, e.g., `Jupyter Notebook`. Case-insensitive. | Empty         |
| `EXCLUDE_FORKS`   | Workflow File     | If `true`, forked repositories will be excluded from statistics.                                                                    | `false`       |
| `EXCLUDE_PRIVATE` | Workflow File     | If `true`, private repositories will be excluded from statistics.                                                                   | `false`       |
| `EXCLUDE_ARCHIVE` | Workflow File     | If `true`, archived repositories will be excluded from statistics.                                                                  | `false`       |
| `LOCALE`          | Workflow File     | Locale for generated images. See [Supported Languages](#supported-languages) for available options.                                 | `en`          |
| `LOG_LEVEL`       | Workflow File     | Log level: `0` (off), `1` (error), `2` (warn), `3` (info), `4` (debug), `5` (trace).                                                | `2`           |
| `DELAY_MS`        | Workflow File     | Delay between API requests in milliseconds to avoid hitting rate limits.                                                            | `1000`        |
| `OUTPUT_BRANCH`   | Workflow File     | The branch name where generated images are stored.                                                                                  | `generated`   |

## Supported Languages

Many strings in the generated images can be localized. You can customize the language used by setting the `LOCALE` configuration. All language codes are case-insensitive, while underscores (`_`) and whitespace (` `) are treated as hyphens (`-`).

| Language Code                       | Language Name        | Status             |
| ----------------------------------- | -------------------- | ------------------ |
| `english`, `en`                     | English              | :heavy_check_mark: |
| `chinese`, `zh`, `zh-cn`, `zh-hans` | Chinese (Simplified) | :heavy_check_mark: |
| `japanese`, `ja`                    | Japanese             | :heavy_check_mark: |
| `french`, `fr`                      | French               | :x:                |
| `german`, `de`                      | German               | :x:                |
| `italian`, `it`                     | Italian              | :x:                |
| `korean`, `ko`                      | Korean               | :x:                |
| `portuguese`, `pt`, `pt-br`         | Portuguese           | :x:                |
| `russian`, `ru`                     | Russian              | :x:                |
| `spanish`, `es`                     | Spanish              | :x:                |

> - :heavy_check_mark: - Proofread and verified by human translators
> - :x: - Machine-translated, may contain errors
>
> If you'd like to contribute translations for unsupported languages or improve existing ones, please see [this file](src/i18n.rs) and consider submitting a pull request!

## Differences from the Original Project

This project is a Rust-based refactor of the original [github-stats](https://github.com/jstrieb/github-stats). Key improvements include:

- **Removed Total Lines of Code**: Since GitHub's API doesn't provide lines of change information for repositories or commits, and as the [original author noted](https://github.com/jstrieb/github-stats#disclaimer), this metric was not very accurate anyway, it has been removed.
- **Added Follower Count**: The total number of followers is now included in the overview stats to better reflect a user's reach on GitHub.
- **Significant Performance Gains**: Despite implementing request throttling and single-threaded processing to avoid API rate limits, generation is much faster. (In testing, for a user with ~100 repositories, the original workflow took about 24 minutes with occasional failures, while this Rust version completes in about 5 minutes reliably.)
- **Modern GitHub API Client**: The project uses the `octocrab` crate, a well-maintained and modern GitHub API client for Rust, ensuring long-term reliability and compatibility with GitHub's API.
- **More Configuration Options**: Added more customization options, such as excluding private repositories from statistics.
- **Dependencies Kept Up-to-Date**: All dependencies are always up-to-date by using [Renovate](https://github.com/renovatebot/renovate), ensuring API compatibility and access to the latest fixes and features.
- **Improved Code Quality and Maintainability**: The codebase has been rewritten in Rust with a focus on readability, maintainability, and performance. Using `octocrab` and the REST API results in cleaner and simpler code compared to the original GraphQL implementation.
- **Cleaner Repository Structure**: The workflow outputs generated images to a separate branch (`generated` by default), keeping the main branch focused on source code with a cleaner history. This makes it easy to sync updates from this template without conflicts from generated files.
- **Template Engine**: This project uses `maud` and `turf` for SVG template rendering and CSS processing, respectively. This provides a more robust and easy-to-tweak system compared to using strings substitution.
- **Minified SVG Outputs**: The SVGs are generated in a minified format, reducing file size by approximately 7% to 10%, which is not massive but still beneficial for faster loading times.

## Notes

- **Logs and Sensitive Information**: If the log level is set to anything other than `0` (off), especially `4` (debug) or higher, workflow logs may contain sensitive information (e.g., private repository names, tokens). Be cautious when sharing logs publicly. These logs are retained by GitHub for a limited time (90 days) and are only visible to you and GitHub administrators.
- **Potential Unexpected Errors**: In rare cases, code errors might inadvertently output sensitive data (e.g., if the API returns an unexpected response causing a parsing library to panic). If you encounter such issues, please report them promptly.
- **Statistics Accuracy**: Limitations in the GitHub API or data changes between requests may cause inaccuracies in the stats. While efforts have been made to minimize this, it cannot be eliminated entirely. It's recommended to periodically check the Actions logs for warnings or errors to assess the reliability of the statistics.

## Development

To run the project locally, you can store configuration like the GitHub Personal Access Token in a `.env` file at the project root. See the [`.env.sample`](.env.sample) file for reference.

This project provides a `justfile` to run the project with these configurations. Make sure you have [Just](https://just.systems/) installed. Then you can use the following commands:

```bash
# Fix code formatting and linting issues
just fix
# Run the project
just run
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project uses only the SVG templates from the original project. All code has been rewritten in Rust with a completely different approach. Nonetheless, it is considered a derivative work of the original. Therefore, following the original project's license, this code is also licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for details.
