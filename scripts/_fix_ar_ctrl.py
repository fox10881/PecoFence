"""Escapes raw U+0004 context markers in ar.json so it parses, then diffs keys vs en.json."""
import io
import json

p = r"locales\ar.json"
s = io.open(p, encoding="utf-8").read()
# The raw control char must be the two-char escape "}" + "u0004" style; build it without
# ever embedding the literal sequence in source.
esc = "\\" + "u0004"
n = s.count(chr(4))
s = s.replace(chr(4), esc)
io.open(p, "w", encoding="utf-8", newline="\r\n").write(s)
print("escaped:", n)

ar = json.load(io.open(p, encoding="utf-8"))
en = json.load(io.open(r"locales\en.json", encoding="utf-8"))
print("en:", len(en), "ar:", len(ar))
missing = [k for k in en if k not in ar]
extra = [k for k in ar if k not in en]
print("missing:", len(missing))
for k in missing:
    print(repr(k))
print("extra:", len(extra))
for k in extra:
    print(repr(k))
