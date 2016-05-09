#!/bin/sh

set -o errexit

git clone --branch gh-pages "https://$GH_TOKEN@github.com/${TRAVIS_REPO_SLUG}.git" deploy_docs
cd deploy_docs

git config user.name "kdy1997"
git config user.email "kdy1997.dev+docbot@gmail.com"


rm -rf .
mv ../target/doc ./
echo "<meta http-equiv=refresh content=0;url=caniuse/index.html>" > ./index.html

git add -A .
git commit -m "rebuild pages at ${TRAVIS_COMMIT}"
git push --quiet origin gh-pages
