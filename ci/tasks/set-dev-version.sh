#!/bin/bash

VERSION="$(cat version/version)-dev"

pushd repo

for file in $(find . -mindepth 2 -name Cargo.toml); do
    sed -i'' "0,/version/{s/version.*/version = \"${VERSION}\"/}" ${file}
done

if [[ -z $(git config --global user.email) ]]; then
  git config --global user.email "bot@cepler.dev"
fi
if [[ -z $(git config --global user.name) ]]; then
  git config --global user.name "CI Bot"
fi

git status
git add -A

if [[ "$(git status -s -uno)" != ""  ]]; then
  git commit -m "ci(dev): set version to ${VERSION}"
fi
