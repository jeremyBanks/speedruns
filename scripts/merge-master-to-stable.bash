current=master
target=remotes/origin/stable

#git checkout $target
#git merge --no-ff $current --log -m "automerge master into stable"
#git push origin $target

# if there's only one new commit, you can fast-forward
# but if there's a break, you need to create a merge commit.
# this way the first-parent ancestor line is of green commits

ff_commits="$(git log --topo-order --format='%H' $current..$target -- | wc -l)"
echo "Number of fast-forwarding commits: $ff_commits"
