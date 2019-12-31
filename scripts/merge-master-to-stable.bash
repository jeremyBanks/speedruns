current=master
target=remotes/origin/stable

git checkout $target

# We want the first-parent ancestor line of the stable branch to
# only include green commits.
if (( "$(git rev-list $target..$current --count)" <= 1 )); then
  # If we're only adding one commit, we can fast-forward it directly.
  git merge --ff --no-edit master
else
  # Otherwise, we need to create a merge commit.
  git merge --no-ff master --log -m "automerge $current into $target"
fi

# fast-forward master to match
git checkout $current
git merge --ff target

git push origin $target $current
