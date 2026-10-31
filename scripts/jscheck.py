import re, sys

src = open(sys.argv[1] if len(sys.argv) > 1 else 'ui/settings.html', encoding='utf-8').read()
blocks = re.findall(r'<script>(.*?)</script>', src, re.S)
big = [b for b in blocks if 'bindAll' in b][0]
s = re.sub(r'//[^\n]*', '', big)
s = re.sub(r'/\*[\s\S]*?\*/', '', s)
s = re.sub(r"'(?:[^'\\]|\\.)*'", "''", s)
s = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
s = re.sub(r'`(?:[^`\\]|\\.)*`', '``', s)
print('braces:', s.count('{') - s.count('}'),
      '| parens:', s.count('(') - s.count(')'),
      '| brackets:', s.count('[') - s.count(']'))
for tok in ['<<<<', '>>>>', 'parent of', 'renderFileExplorerStyles',
            'renderColorTint', 'bindColorTint', 'colorTintRow', 'fileExplorerStyleSel']:
    print(tok.ljust(26), big.count(tok))
