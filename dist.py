""" Probably not _really_ representative of the distribution 
of keywords in Python code, but it's a start. 

To save you the trouble:

Keyword   | Count | Frequency (%)
----------|-------|--------------
def       | 38033 |         3.35
if        | 21472 |         1.89
return    | 16071 |         1.42
None      | 15039 |         1.33
in        | 11549 |         1.02
for       |  8693 |         0.77
class     |  8680 |         0.77
with      |  7709 |         0.68
import    |  7446 |         0.66
not       |  6577 |         0.58
else      |  5742 |         0.51
True      |  5121 |         0.45
raise     |  4922 |         0.43
try       |  4888 |         0.43
is        |  4813 |         0.42
False     |  4387 |         0.39
as        |  4225 |         0.37
except    |  4093 |         0.36
pass      |  3507 |         0.31
from      |  3162 |         0.28
and       |  2815 |         0.25
elif      |  2071 |         0.18
or        |  1759 |         0.16
del       |  1228 |         0.11
yield     |  1096 |         0.10
finally   |  1087 |         0.10
lambda    |  1055 |         0.09
while     |   975 |         0.09
assert    |   841 |         0.07
break     |   736 |         0.06
continue  |   618 |         0.05
async     |   319 |         0.03
await     |   201 |         0.02
global    |   188 |         0.02
nonlocal  |   112 |         0.01

The idea here is to use this information when creating the match arms. Since Rust 
compiles a match as a jump table, the more common keywords should be at the top of
the match arms. This will improve the detection of keywords (is my suspicion).
"""
import sys
import keyword
from collections import Counter

def read_keywords(filename):
    with open(filename, 'r') as file:
        keywords = file.read().split()
    return keywords

def print_frequency_distribution(keywords):
    counter = Counter(keywords)
    total_keywords = len(keywords)
    
    print("Keyword   | Count | Frequency (%)")
    print("----------|-------|--------------")
    for word, count in counter.most_common():
        if keyword.iskeyword(word):
            frequency = count / total_keywords * 100
            print(f"{word:10}| {count:5} |{frequency:13.2f}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python dist.py <filename>")
        sys.exit(1)

    filename = sys.argv[1]
    keywords = read_keywords(filename)
    print_frequency_distribution(keywords)