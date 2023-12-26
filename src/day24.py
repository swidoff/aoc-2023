import re
from collections import Counter
from pathlib import Path
from typing import List

import numpy as np
from scipy.optimize import fsolve


def read_file():
    with open("../input/day24.txt", "r") as f:
        lines = f.readlines()

    return [
        [float(v) for v in re.split(",|@", line.strip())]
        for line in lines
    ]


def solve(p1: List[float], p2: List[float], p3: List[float], start: List[float]) -> List[float]:
    [ax, ay, az, axv, ayv, azv] = p1
    [bx, by, bz, bxv, byv, bzv] = p2
    [cx, cy, cz, cxv, cyv, czv] = p3

    def f(vars):
        px, py, pz, pxv, pyv, pzv, t1, t2, t3 = vars
        return [
            (px + pxv * t1) - (ax + axv * t1),
            (py + pyv * t1) - (ay + ayv * t1),
            (pz + pzv * t1) - (az + azv * t1),
            (px + pxv * t2) - (bx + bxv * t2),
            (py + pyv * t2) - (by + byv * t2),
            (pz + pzv * t2) - (bz + bzv * t2),
            (px + pxv * t3) - (cx + cxv * t3),
            (py + pyv * t3) - (cy + cyv * t3),
            (pz + pzv * t3) - (cz + czv * t3),
        ]

    return fsolve(f, start, maxfev=1_000_000, factor=1)


def part2(p: List[List[float]]):
    """
    ax + axv*t1 = px + pxv*t1 => ax + axv*t1 - (px + pxv*t1) = 0
    ay + ayv*t1 = py + pyv*t1 => ay + ayv*t1 - (py + pyv*t1) = 0
    az + azv*t1 = pz + pzv*t1 => az + azv*t1 - (pz + pzv*t1) = 0

    bx + bxv*t1 = px + pxv*t2
    by + byv*t1 = py + pyv*t2
    bz + bzv*t1 = pz + pzv*t2

    cx + cxv*t1 = px + pxv*t3
    cy + cyv*t1 = py + pyv*t3
    cz + czv*t1 = pz + pzv*t3
    """
    counter = Counter()

    res = [0.0] * 9
    rng = np.random.default_rng()
    counter = Counter()
    for _ in range(100):
        rng.shuffle(p)
        for i in range(len(p) - 2):
            res = solve(p[i], p[i + 1], p[i + 2], res)
            counter[round(res[0] + res[1] + res[2])] += 1

    res = round(res[0] + res[1] + res[2])
    print(res)
    print(counter.most_common(10))


if __name__ == '__main__':
    p = read_file()
    part2(p)
