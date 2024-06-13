#! /usr/bin/env -S bs
# set -x
# dir="$XDG_RUNTIME_DIR/test"
# mkdir -p "$dir"
# cd "$dir"
# echo "hello" > file
# python3 "$FILE"

with open("file", "r") as f:
    contents = f.read()
    print(contents)

