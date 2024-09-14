#!/bin/python3

words = dict()
simp = set()

def uplus(u):
    return int(u[2:], 16)

with open("Unihan_Variants.txt", "r") as file:
    for line in file.readlines():
        tabs = line.split("\t")

        lhs = uplus(tabs[0])
        rhs = [uplus(s) for s in tabs[2].split()]

        if rhs != [] and rhs != [lhs]:
            simp.add(chr(lhs))

with open("cangjie5.dict.txt", "r") as file:
    for line in file.readlines():
        tabs = line.split("\t")

        if tabs[0] in simp or 0x2e80 <= ord(tabs[0]) <= 0x2eff:
            continue

        if tabs[0] not in words:
            words[tabs[0]] = set()

        words[tabs[0]] = words[tabs[0]] | set(['"' + t.strip().replace("tlc", "tmc") + '"' for t in filter(lambda c: not (c.startswith("x") or c.startswith("yyy") or ("z" in c) or ("[" in c)), tabs[1:])])

with open("src/lib.rs", "w") as out:
    out.write("pub static CHARS: &[(char, &[&str])] = &[\n")
    for c, e in words.items():
        if len(e) != 0:
            out.write(f"('{c}', &[{','.join(e)}]),\n")
    out.write("];\n")
