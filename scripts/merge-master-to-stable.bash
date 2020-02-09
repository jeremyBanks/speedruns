#!/bin/bash
set -euxo pipefail

current=master
target=deploy

git checkout $target

# We want the first-parent ancestor line of the target branch to
# only include green commits.
if (( "$(git rev-list $target..$current --count)" <= 1 )); then
  # If we're only adding one commit, we can fast-forward it directly.
  git merge --ff --no-edit $current
else
  # Otherwise, we need to create a merge commit.
  git merge --no-ff $current -m "$(git log $target..$current --format="format:%B%n")"
fi

# fast-forward current to match
git checkout $current
git merge --ff $target

if git push origin $current; then
  git push -f origin $target
else
  echo 'failed to push to -- maybe this has been superseded by a subsequent push?'
  false
fi
