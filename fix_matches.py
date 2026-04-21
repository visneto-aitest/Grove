#!/usr/bin/env python3
import re

with open(
    "/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/main.rs", "r"
) as f:
    content = f.read()

lines = content.split("\n")
i = 0
while i < len(lines):
    line = lines[i]
    if "match provider" in line and "{" in line:
        brace_count = line.count("{") - line.count("}")
        j = i
        while brace_count > 0 and j + 1 < len(lines):
            j += 1
            brace_count += lines[j].count("{") - lines[j].count("}")

        block = "\n".join(lines[i : j + 1])
        if "Beads" not in block:
            indent = len(lines[j]) - len(lines[j].lstrip())
            lines.insert(j, " " * indent + "ProjectMgmtProvider::Beads => todo!(),")
            i = j + 2
        else:
            i = j + 1
    else:
        i += 1

with open(
    "/Volumes/MacMini4-ssd/home/Users/kong/code_aicoder/Grove/src/main.rs", "w"
) as f:
    f.write("\n".join(lines))

print("Done adding Beads match arms")
