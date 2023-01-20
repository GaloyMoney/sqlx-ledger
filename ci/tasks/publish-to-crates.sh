#!/bin/bash

set -e

VERSION="$(cat version/version)"

pushd repo

git checkout "${VERSION}"

cat <<EOF | cargo login
${CRATES_API_TOKEN}
EOF

cargo publish -p cel-parser --all-features --no-verify
cargo publish -p cel-interpreter --all-features --no-verify
cargo publish -p ledger --all-features --no-verify
