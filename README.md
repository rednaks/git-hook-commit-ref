# Description
Git hook to prepare commit message.

Will check if you're allowed to commit or not, if you're allowed to commit, check if there is a reference that matches the branch
if not it will update the commit message.

# How to use

build the project:
```
cargo build --release
```

## Configure your git repo

in your project, edit the `.git/config` file to add the organization and the project name, and a list of forbidden branches to commit from.

eg:

```
[commit-ref-hook]
  org = "rednaks"
  project = "git-hook-commit-ref"
  forbiddenbranches = "master, release"
```

and then copy the binary to you `.git/hooks/prepare-commit-msg`

eg:
```sh
cp git-hook-commit-ref/target/release/git-hook-commit-ref  my_git_project/.git/hooks/prepare-commit-msg
```
