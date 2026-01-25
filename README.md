# GitHub stats

Generate GitHub statistics images for your profile README, updated daily.

This project is modified from [github-stats](https://github.com/jstrieb/github-stats) by [jstrieb](https://github.com/jstrieb). Key differences and improvements are listed in the [Differences from the Original Project](#differences-from-the-original-project) section.

![GitHub Stats for TeddyHuang-00 (Dark Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-dark-mode-only)
![GitHub Stats for TeddyHuang-00 (Light Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/overview.svg#gh-light-mode-only)
![Languages Stats for TeddyHuang-00 (Dark Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-dark-mode-only)
![Languages Stats for TeddyHuang-00 (Light Mode)](https://raw.githubusercontent.com/TeddyHuang-00/github-stats-rs/generated/languages.svg#gh-light-mode-only)

## Step-by-step Guide

1. Set up a GitHub Personal Access Token (PAT) with `repo`, `read:user` and `read:org` scopes.
   1. Go to [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens).
   2. Create a new token with the required scopes. **Make sure to copy the token as you won't be able to see it again.**
2. Use this repository as a template to create your own repository.
   1. Click the **"Use this template"** button in the top right corner of this page. Then select **"Create a new repository"**.
   2. Fill in the repository name (e.g., `github-stats-rs`, or whatever you prefer).
   3. Click **"Create repository"** to confirm.
3. Go to your newly created repository's **Settings > Secrets and variables > Actions**.
   1. Click **"New repository secret"**.
   2. Set the name to `ACCESS_TOKEN` and paste your PAT as the value. Click **"Add secret"** to save.
   3. _(Optional)_ If you want to customize the behavior, you can also create some other secrets. See the [Configuration](#configuration) section for more details.
4. _(Optional)_ Configure some other settings by editing the workflow file located at `.github/workflows/generate.yml`.
   1. You can change the schedule of the workflow, or modify the commit message.
   2. Some other options are also available. See the [Configuration](#configuration) section for more details.
5. Once everything is set up, you can manually trigger the workflow or wait for the scheduled time.
   1. To manually trigger, go to the **Actions** tab in your repository, select the **"Generate Stats Images"** workflow, and click the **"Run workflow"** button.
   2. Wait for the workflow to complete. Once done, the generated images will be available in the `generated` branch of your repository.
6. Finally, add the generated images to your profile README by including the following Markdown code:

   ```markdown
   ![GitHub Stats for YOUR_USERNAME (Dark Mode)](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPOSITORY/generated/overview.svg#gh-dark-mode-only)
   ![GitHub Stats for YOUR_USERNAME (Light Mode)](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPOSITORY/generated/overview.svg#gh-light-mode-only)
   ![Languages Stats for YOUR_USERNAME (Dark Mode)](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPOSITORY/generated/languages.svg#gh-dark-mode-only)
   ![Languages Stats for YOUR_USERNAME (Light Mode)](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPOSITORY/generated/languages.svg#gh-light-mode-only)
   ```

   > Replace `YOUR_USERNAME` and `YOUR_REPOSITORY` with your actual GitHub username and repository name.
   >
   > If you change the branch name in the workflow file, make sure to update the URLs accordingly.

7. Enjoy your dynamic GitHub stats images!

## Configuration

Here are some optional configurations you can set up by adding repository secrets or modifying the workflow file:

> All the optional configurations can be removed if you want to use the default settings.
>
> You can also move some configurations between repository secrets and workflow file as you prefer, by changing the workflow file to use secrets or vice versa. Secrets are more secure, while workflow file is easier to edit. You should use secrets for whatever you consider to be sensitive, and **DEFINITELY for the `ACCESS_TOKEN`**.

| Configuration Option | Place           | Description                                                                                                                                    | Default Value  |
| -------------------- | --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| `ACCESS_TOKEN`       | Actions secrets | Your GitHub Personal Access Token (PAT) with `repo`, `read:user`, and `read:org` scopes.                                                       | N/A (Required) |
| `EXCLUDE_REPOS`      | Actions secrets | Comma-separated list of repository names to exclude from statistics. Should be full names, i.e., `username/repository`.                        | Empty          |
| `EXCLUDE_LANGS`      | Actions secrets | Comma-separated list of programming languages to exclude from statistics. Should be display names, e.g., `Jupyter Notebook`. Case-insensitive. | Empty          |
| `EXCLUDE_FORKS`      | Workflow file   | If `true`, forked repositories will be excluded from statistics.                                                                               | `false`        |
| `EXCLUDE_PRIVATE`    | Workflow file   | If `true`, private repositories will be excluded from statistics.                                                                              | `false`        |
| `EXCLUDE_ARCHIVE`    | Workflow file   | If `true`, archived repositories will be excluded from statistics.                                                                             | `false`        |
| `LOG_LEVEL`          | Workflow file   | Set the log level for the workflow. Options are `0` (off), `1` (error), `2` (warn), `3` (info), `4` (debug), `5` (trace).                      | `2`            |
| `DELAY_MS`           | Workflow file   | Delay between API requests in milliseconds to avoid hitting rate limits.                                                                       | `1000`         |
| `OUTPUT_BRANCH`      | Workflow file   | The branch where the generated images will be pushed.                                                                                          | `generated`    |

## Differences from the Original Project

This project is a Rust adaptation of the original [github-stats](https://github.com/jstrieb/github-stats) project. Here are some key differences:

- **Deleted total lines of code**: As GitHub's API doesn't seem to provide information on lines of changes even for each commit in a repository, this is excluded from the statistics. As [mentioned by the original author](https://github.com/jstrieb/github-stats#disclaimer), this metric was not very accurate anyway.
- **Added followers count**: The total number of followers is now displayed in the overview statistics, to give a better sense of the user's reach on GitHub.
- **Performance improvements**: Leveraging Rust's performance capabilities, this version aims to be faster, despite using throttling and only running single requests concurrently to avoid hitting GitHub's rate limits. (In my tests, the original workflow took around 24 minutes with failures, while this Rust version took around 5 minutes in total without failures for the same user with ~100 repositories.)
- **Modern GitHub API client in Rust**: The `octocrab` crate is used for the heavy lifting. It is a modern and ergonomic GitHub API client for Rust, and receives active maintenance. Meaning better long-term reliability for this project as long as the crate is up-to-date.
- **Extra configuration options**: Additional configuration options have been added, such as the ability to exclude private repositories from statistics.
- **Updated dependencies**: All dependencies are always kept up-to-date using [Renovate](https://github.com/renovatebot/renovate), ensuring API compatibility and access to the latest features and bug fixes.
- **Improved code quality**: The codebase has been rewritten in Rust, focusing on maintainability, readability, and performance. Notably, by using `octocrab` and REST API, the code is significantly simplified compared to the original project which used GraphQL API.
- **Cleaner Git history**: The workflow is designed to separate the generated images into a different branch (`generated` by default), keeping the main branch clean and focused on the code. You can easily merge changes from this repository template into your own repository without worrying about generated files cluttering the history. This way, you can keep your fork up-to-date with the latest improvements from this project.

## Disclaimer

If you set the log level to anything other than `0` (off), especially `4` (debug) or higher, the actions workflow logs may contain some possibly sensitive information, such as private repository names or even access tokens. Please be cautious when sharing logs publicly. These logs are only stored by GitHub for a limited time (90 days as of June 2024) and are not accessible to anyone other than you and GitHub staff.

Although very unlikely, there also might be cases where the code inadvertently runs into an error and prints sensitive information. For example, if the GitHub API returns an unexpected response that includes sensitive data, and the `octocrab` crate panics while trying to parse it, the panic message might include the raw response data. If you notice any such issues, please report them so they can be addressed promptly.

Inaccurate statistics may occur due to limitations in the GitHub API or changes in repository data between requests. While efforts have been made to minimize these issues, they cannot be completely eliminated. I recommend reviewing the actions logs periodically to check for warnings or errors that indicate potential inaccuracies. For example, if you see warnings about unable to get certain repository information, it may affect the accuracy of the statistics.

## Development

To run the project locally with configuration like the GitHub Personal Access Token, you can store them in a `.env` file at the root of the project. See the [`.env.sample`](.env.sample) file for reference.

You can use the `justfile` provided in the repository to run the project with these configurations. Make sure you have [Just](https://just.systems/) installed. Then, you can use the following commands:

```bash
# Fix format and linting issues
just fix
# Run the project
just run
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project only utilizes SVG templates from the original project, while all the codebase has been rewritten in Rust with a completely different approach. Still, I consider this project to be a derivative work of the original. Therefore, as per the original project, this code is licensed under the GPLv3 License. Please refer to the [LICENSE](LICENSE) file for more details.
