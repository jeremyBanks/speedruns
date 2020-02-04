#!/bin/bash
set -euxo pipefail;

echo "//npm.pkg.github.com/:_authToken=${GITHUB_PUBLISH_TOKEN}" > .npmrc
echo "@jeremybanks:registry=https://npm.pkg.github.com" >> .npmrc
echo "always-auth=true" >> .npmrc

npm version prerelease --no-git-tag-version --preid="r$(git rev-list --first-parent HEAD | wc -l)-$(git rev-parse --short=4 HEAD)"

NODE_AUTH_TOKEN=${GITHUB_PUBLISH_TOKEN} npm --registry=https://npm.pkg.github.com/:_authToken=${GITHUB_PUBLISH_TOKEN} publish
NODE_AUTH_TOKEN=${NPM_PUBLISH_TOKEN} npm --registry=https://registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN} publish

sed -i 's/@jeremybanks\///' package.json

NODE_AUTH_TOKEN=${NPM_PUBLISH_TOKEN} npm --registry=https://registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN} publish

npm --registry=https://npm.pkg.github.com/:_authToken=${GITHUB_PUBLISH_TOKEN} publish

rm .npmrc

echo "//registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN}" > .npmrc

npm --registry=https://registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN} publish

sed -i 's/@jeremybanks\///' package.json

npm --registry=https://registry.npmjs.org/:_authToken=${NPM_PUBLISH_TOKEN} publish
