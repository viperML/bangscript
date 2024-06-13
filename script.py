#! /usr/bin/env -S bs -p bash
#eval "$(nix print-dev-env --impure --expr "$(cat <<EOF
#with import <nixpkgs> {};
#mkShell {
#    packages = [
#        (python3.withPackages (p: [
#          p.requests
#        ]))
#    ];
#}
#EOF
#)")"
#python3 "$FILE"

import requests

resp = requests.get("https://httpbin.org")
print(resp)

