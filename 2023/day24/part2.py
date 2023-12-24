"""
Python solution using sympy
"""
from sympy import Eq, var, solve

class HailStone:
    position: (int, int, int)
    velocity: (int, int, int)

    def __init__(self, position, velocity):
        self.position = position
        self.velocity = velocity


hail_stones = []
for l in open("./input.txt"):
    p = l.split(" @ ")
    position = list(map(int, p[0].split(", ")))
    velocity = list(map(int, p[1].split(", ")))

    hail_stones.append(HailStone(position, velocity))

# Part 2

px = var("px")
py = var("py")
pz = var("pz")

vx = var("vx")
vy = var("vy")
vz = var("vz")

eqs = []
for (i, hail_stone) in enumerate(hail_stones):
    if i > 3:
        # Knowing 3 hail stones is enough
        break

    p = hail_stone.position
    v = hail_stone.velocity

    t = var(f"t_{i}")
    eqs.append(Eq(px + vx * t, p[0] + v[0] * t))
    eqs.append(Eq(py + vy * t, p[1] + v[1] * t))
    eqs.append(Eq(pz + vz * t, p[2] + v[2] * t))

answer = solve(eqs)[0]
print(answer)
print(answer[px] + answer[py] + answer[pz])
