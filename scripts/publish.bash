#!/bin/bash
set -euxo pipefail;

echo "//npm.pkg.github.com/:_authToken=${GITHUB_PUBLISH_TOKEN}" > .npmrc
echo "@jeremybanks:registry=https://npm.pkg.github.com" >> .npmrc
echo "always-auth=true" >> .npmrc

npm version prerelease --no-git-tag-version --preid="dev-r$(git rev-list --first-parent HEAD | wc -l)"
sed -i '0,/\.0"/ s/\.0"/"/' package.json

npm --registry=https://npm.pkg.github.com/ publish

echo "//registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN}" > .npmrc

npm --registry=https://registry.npmjs.org/ publish

sed -i 's/@jeremybanks\///' package.json

npm --registry=https://registry.npmjs.org/ publish
