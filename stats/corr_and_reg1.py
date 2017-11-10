from itertools import product
from math import fsum, sqrt

def difs(x):
    return [xi - xj for xi, xj in product(x, x)]

def cov(x, y):
    assert len(x) == len(y)
    return 0.5 * fsum([xd * yd for xd, yd in zip(difs(x), difs(y))]) / len(x)**2

def stdev(x):
    return sqrt(cov(x, x))

def pear(x, y):
    return cov(x, y) / stdev(x) / stdev(y)

def out(x):
    print('{0:.3f}'.format(round(x, 3)))

x = [int(xi) for xi in '15  12  8   8   7   7   7   6   5   3'.split('  ')]
y = [int(xi) for xi in '10  25  17  11  13  17  20  13  9   15'.split('  ')]

out(pear(x, y))
