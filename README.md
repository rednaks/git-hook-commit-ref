# Description
Git hook to prepare commit message.

Will check if you're allowed to commit or not, if you're allowed to commit, check if there is a reference that matches the branch
if not it will update the commit message.

# How to use

build the project:
```
cargo install git-hook-commit-ref
```

# Install the hook to your current git repo:
```sh
cd my_git_project
git hook-commit-ref --install
```
## Configure your git repo

in your project, edit the `.git/config` file to add the organization and the project name, and a list of forbidden branches to commit from.

eg:

```
[commit-ref-hook]
  org = "rednaks"
  project = "git-hook-commit-ref"
  forbiddenbranches = "master, release"
  branchpattern = "(?P<org>\w+).*-(?P<issue_number>\d+).*"
```

## Branch name
the default branch name should match the `<org>-<issue_number>` pattern, but if your branch name is different, make sure to add the regex matching your branch name.
don't forget to add `org` and `issue_number` to capture the matches.

`org` is optional, but `issue_number` is required.

### Check everything is good
To check that everything works and configured, you can use `git hook-commit-ref --check` in your git repo.


### Bypassing hook
In some situations like merging or rebasing there is no current branch, but a reference instead. you won't be able to commit your changes because it doesn't match the hook's configuration.
You can use `COMMIT_HOOK_IGNORE=true` env var when commit to bypass the hook.
