#!/bin/bash
set -veuxo pipefail

current=master
target=✔️

git checkout $target

# We want the first-parent ancestor line of the stable branch to
# only include green commits.
if (( "$(git rev-list $target..$current --count)" <= 1 )); then
  # If we're only adding one commit, we can fast-forward it directly.
  git merge --ff --no-edit $current --allow-empty-message
else
  # Otherwise, we need to create a merge commit.
  git merge --no-ff $current -m "$(git log ✔️..$current --format="format:%B%n")" --allow-empty-message
fi

# fast-forward master to match
git checkout $current
git merge --ff $target

if git push origin $current; then
  git push -f origin $target
fi
