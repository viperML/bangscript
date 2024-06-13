#! /usr/bin/env -S bs
# eval "$(guix shell python --search-paths)"
# python3 "$FILE"

import sysconfig
print(sysconfig.get_config_vars()["prefix"])

