"""Check native --test-script fold dumps, including stale-size and reversal probes."""
import json
import datetime
import re
import statistics
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
assert "ERROR" not in text, "Native render error"
states = []
for line in text.splitlines():
    if 'title="Lens-Test"' not in line or " INFO pecofence::test:" not in line:
        continue
    tag = re.search(r"\[([^]]+)\] live hwnd=", line)
    if not tag:
        continue
    client = tuple(map(int, re.search(r"client=\((\d+), (\d+)\)", line).groups()))
    chrome = tuple(map(int, re.search(r"chrome=\((\d+), (\d+)\)", line).groups()))
    backing = tuple(map(int, re.search(r"chrome_backing=\(\((\d+), (\d+)\), (\d+)\)", line).groups()))
    content = tuple(map(int, re.search(r"content_surface=\((\d+), (\d+)\)", line).groups()))
    alpha = float(re.search(r"content_opacity=([\d.]+)", line)[1])
    progress = float(re.search(r"roll_progress=([\d.]+)", line)[1])
    rolling = "rolling=true" in line
    rolled = "rolled=true" in line
    assert client == chrome, (tag[1], client, chrome)
    assert backing[0] == client[0] and backing[1] >= client[1], (tag[1], backing)
    assert 0 <= alpha <= 1
    if not rolling:
        assert progress == float(rolled), (tag[1], progress, rolled)
        assert alpha == 1, (tag[1], alpha)
    if tag[1].startswith("reverse-"):
        assert not rolling and not rolled, (tag[1], "reversal did not finish expanded")
    states.append(dict(tag=tag[1], client=client, backing=backing, content=content,
                       alpha=alpha, rolling=rolling))

assert len([s for s in states if s["tag"].startswith("reverse-")]) == 20
assert len([s for s in states if s["tag"].startswith("stale-")]) == 20
assert len({tuple(s["backing"]) for s in states}) == 1, "Drawing surface resized during folding"
assert len({tuple(s["content"]) for s in states}) == 1, "Expanded icon raster was truncated"
assert any(s["rolling"] and 0 < s["alpha"] < 1 for s in states), "No intermediate fade sample"
report = dict(states=len(states), reversal_cycles=20, stale_size_probes=20,
              drawing_surface_stable=True, expanded_raster_stable=True,
              viewport_matches_window=True, content_fade_sampled=True)
intervals = []
previous = None
for line in text.splitlines():
    if "height frame" not in line or "rolled=Some(false)" not in line:
        if "height frame" in line:
            previous = None
        continue
    now = datetime.datetime.fromisoformat(line.split()[0].replace("Z", "+00:00"))
    height = int(re.search(r"height=(\d+)", line)[1])
    if previous and previous[1] != height:
        intervals.append((now - previous[0]).total_seconds() * 1000)
    previous = None if "done=true" in line else (now, height)
if intervals:
    intervals.sort()
    report["height_update_timing"] = dict(samples=len(intervals),
        median_ms=round(statistics.median(intervals), 2),
        p95_ms=round(intervals[int(len(intervals) * .95)], 2), max_ms=round(max(intervals), 2))
print(json.dumps(report, indent=2))
