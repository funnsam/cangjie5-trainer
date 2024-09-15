#!/bin/python3

import sys
from read import get_words

with open(sys.argv[1], "w") as out:
    out.write("const chars = [\n")
    for c, e in get_words().items():
        if len(e) != 0:
            out.write(f"['{c}', [{','.join(e)}]],\n")
    out.write("];\n")
