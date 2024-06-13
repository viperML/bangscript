#! /usr/bin/env bash

eval "$(nix print-dev-env --impure --expr "$(cat <<EOF
with import <nixpkgs> {};
mkShell {
    packages = [
        python3
    ];
}
EOF
)")"

python3 --version
