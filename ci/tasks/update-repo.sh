#!/bin/bash

set -eu

# ----------- UPDATE REPO -----------
git config --global user.email "bot@galoy.io"
git config --global user.name "CI Bot"

pushd repo

VERSION="$(cat ../version/version)"

cat <<EOF >new_change_log.md
# [sqlx-ledger release v${VERSION}](https://github.com/GaloyMoney/sqlx-ledger/releases/tag/v${VERSION})

$(cat ../artifacts/gh-release-notes.md)

$(cat CHANGELOG.md)
EOF
mv new_change_log.md CHANGELOG.md

for file in $(find . -mindepth 2 -name Cargo.toml); do
    sed -i'' "s/^version.*/version = \"${VERSION}\"/" ${file}
done

sed -i'' "s/cel-parser\", version = .*/cel-parser\", version = \"${VERSION}\" }/" cel-interpreter/Cargo.toml
sed -i'' "s/cel-interpreter\", version = .*/cel-interpreter\", version = \"${VERSION}\" }/" ledger/Cargo.toml

git status
git add .

if [[ "$(git status -s -uno)" != ""  ]]; then
  git commit -m "ci(release): release version $(cat ../version/version)"
fi
