#!/bin/sh

set -o errexit

RUSTDOC_REPO="rustdoc"
RUSTDOC_REPO_SLUG="kdy1997/rustdoc"

git clone --branch gh-pages "https://$GH_TOKEN@github.com/${RUSTDOC_REPO_SLUG}.git" deploy_docs
cd deploy_docs

git config user.name "kdy1997"
git config user.email "kdy1997.dev+docbot@gmail.com"


rm -rf caniuse/
mv ../target/doc caniuse
echo "<meta http-equiv=refresh content=0;url=caniuse/index.html>" > ./caniuse/index.html

git add -A .
git commit -m "rebuild pages at ${TRAVIS_COMMIT}"
git push --quiet origin gh-pages
