#!/bin/python3

import hanzidentifier

words = dict()

with open("cangjie5.dict.txt", "r") as file:
    for line in file.readlines():
        tabs = line.split("\t")

        if not hanzidentifier.is_simplified(tabs[0]):
            if tabs[0] not in words: words[tabs[0]] = set()

            words[tabs[0]] = words[tabs[0]] | set(['"' + t.strip().replace("tlc", "tmc") + '"' for t in filter(lambda c: not (c.startswith("x") or ("z" in c) or ("[" in c)), tabs[1:])])

with open("src/lib.rs", "w") as out:
    out.write("pub static CHARS: &[(char, &[&str])] = &[")
    for c, e in words.items():
        if len(e) != 0:
            out.write(f"('{c}', &[{','.join(e)}]),")
    out.write("];")

    out.write(f"pub static ALL: &str = \"{''.join([w for w, _ in words.items()])}\";")
