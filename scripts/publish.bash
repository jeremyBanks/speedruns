#!/bin/bash
set -veuo pipefail

if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    publish_args=
else
    echo "NOT PUBLISHING -- doing a local dry run"
    publish_args=--dry-run
    GITHUB_PUBLISH_TOKEN=
    NPM_PUBLISH_TOKEN=
    CARGO_PUBLISH_TOKEN=
fi

echo "//npm.pkg.github.com/:_authToken=${GITHUB_PUBLISH_TOKEN}" > .npmrc
echo "@jeremybanks:registry=https://npm.pkg.github.com" >> .npmrc
echo "always-auth=true" >> .npmrc

npm version prerelease --no-git-tag-version --preid="dev.$((5 + $(git rev-list --first-parent HEAD | wc -l)))"
sed -i '0,/\.0"/ s/\.0"/"/' package.json

version="$(cat package.json | $(yarn bin jqn) 'property("version")')"

sed -i '0,/version = ".*"/ s/version = ".*"/version = "'$version'"/' Cargo.toml

git diff

git tag "$version"
git push origin "$version"

echo "$version" > .version

npm --registry=https://npm.pkg.github.com/ publish $publish_args

echo "//registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN}" > .npmrc

npm --registry=https://registry.npmjs.org/ publish $publish_args

sed -i 's/@jeremybanks\///' package.json

git diff

npm --registry=https://registry.npmjs.org/ publish $publish_args

cargo publish $publish_args --no-verify --token "$CARGO_PUBLISH_TOKEN" --allow-dirty

rm .npmrc
git checkout HEAD package.json Cargo.toml Cargo.lock
