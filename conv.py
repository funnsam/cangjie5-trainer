#!/bin/python3

import hanzidentifier

print("pub static CHARS: &[(char, &[&str])] = &[")
words = []

with open("cangjie5.dict.yaml", "r") as file:
    for line in file.readlines():
        tabs = line.split("\t")

        if hanzidentifier.is_traditional(tabs[0]) and not any(map(lambda c: c.startswith("x"), tabs[1:])):
            print(f"('{tabs[0]}', &[{', '.join(['"' + t.strip() + '"' for t in tabs[1:]])}]),")
            words.append(tabs[0])

print("];")
print(f"pub static ALL: &str = \"{"".join(words)}\";")
