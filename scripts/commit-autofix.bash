#!/bin/bash
set -veuxo pipefail

git add .

if ! [[ -z "$(git status -s -uall)" ]]; then

    echo "$(git diff --unified=0 HEAD | grep '^[\+\-]' | wc -l) lines modified."

    # copy message and authorship from current (parent) commit
    # but use distinct committer and commit timestamp
    GIT_AUTHOR_DATE="$(git log -1 --pretty=format.bash:'%ad')" \
      git commit -m $"[autofix] $(git log -1 --pretty=%B)" --allow-empty-message

    git push || echo "WARNING: Failed to push."
fi
