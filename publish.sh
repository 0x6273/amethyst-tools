#!/bin/bash

# Publish the book and the API documentation to the `gh-pages' branch.

mkdir web
cp -r target/doc/* web/

sudo pip install ghp-import
ghp-import -n web/

git config user.name "Eyal Kalderon"
git config user.email "ebkalderon@gmail.com"
git push -fq https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
