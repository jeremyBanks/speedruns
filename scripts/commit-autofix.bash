#!/bin/bash
set -veuxo pipefail

if ! [[ -z "$(git status -s -uall)" ]]; then
    git add .

    # copy message and authorship from current (parent) commit
    # but use distinct committer and commit timestamp
    GIT_AUTHOR_DATE="$(git log -1 --pretty=format.bash:'%ad')" \
      git commit -m $"[autofix] $(git log -1 --pretty=%B)" --allow-empty-message

    git push
fi
