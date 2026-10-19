import json

path = r"C:\PecoFence-0.2.3\PecoFence-0.2.3\site\schema\config.json"
j = json.load(open(path, encoding="utf-8"))
d = j["$defs"]
for k in d:
    if "labelLines" in json.dumps(d[k]) or "iconSize" in json.dumps(d[k]):
        print(k, "=>", json.dumps(d[k], ensure_ascii=False))
