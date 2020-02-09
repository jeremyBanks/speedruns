#!/bin/bash
set -euxo pipefail

git add .

if ! [[ -z "$(git status -s -uall)" ]]; then
    # copy message and authorship from current (parent) commit
    # but use distinct committer and commit timestamp
    GIT_AUTHOR_DATE="$(git log -1 --pretty=format.bash:'%ad')" \
      git commit -m $"[autofix] $(git log -1 --pretty=%B)" --allow-empty-message

    git push || echo "WARNING: Failed to push."
fi
