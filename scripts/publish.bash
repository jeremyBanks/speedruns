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

### Versioning

npm version prerelease --no-git-tag-version --preid="dev.$(($(git rev-list --first-parent 0.20.20...HEAD | wc -l) - 0))"
sed -i '0,/\.0"/ s/\.0"/"/' package.json

version="$(cat package.json | $(yarn bin jqn) 'property("version")')"

yarn replace-in-files --regex='\nversion = "(.*)"\n' --replacement='\nversion = "'$version'"\n' Cargo.toml src/lib/*/Cargo.toml
yarn replace-in-files --regex='(\nspeedruns_[a-z]+ = .*?version = )".*?"(.*\n)' --replacement='$1"'$version'"$2' Cargo.toml src/lib/*/Cargo.toml

export NODE_ENV=production
yarn build

git diff

git tag "$version"
git push origin "$version"

echo "$version" > .version

### Cargo

(cd src/lib/utils && cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty && cd -)
sleep 60
(cd src/lib/models && cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty && cd -)
sleep 60
(cd src/lib/database && cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty && cd -)
sleep 60
(cd src/lib/juniper && cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty && cd -)
sleep 60
(cd src/lib/api && cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty && cd -)
sleep 60
(cd src/lib/cli && cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty && cd -)
sleep 60
cargo publish $publish_args --token "$CARGO_PUBLISH_TOKEN" --allow-dirty

### NPM

rm -r .next/cache

echo "//npm.pkg.github.com/:_authToken=${GITHUB_PUBLISH_TOKEN}" > .npmrc
echo "@jeremybanks:registry=https://npm.pkg.github.com" >> .npmrc
echo "always-auth=true" >> .npmrc

npm --registry=https://npm.pkg.github.com/ publish $publish_args

echo "//registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN}" > .npmrc

npm --registry=https://registry.npmjs.org/ publish $publish_args

sed -i 's/@jeremybanks\///' package.json

git diff

npm --registry=https://registry.npmjs.org/ publish $publish_args

rm .npmrc
git checkout HEAD package.json Cargo.toml Cargo.lock
