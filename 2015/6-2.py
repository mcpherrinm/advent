import re
grid = map(lambda x: x[:],[[0]*1000]*1000) 

def apply(grid, sx, sy, ex, ey, action):
  for i in xrange(sx, ex+1):
    for j in xrange(sy, ey+1):
      new = action(grid[i][j])
      grid[i][j] = new

def turnon(x):
  return x+1

def turnoff(x):
  return max(0, x-1)

def toggle(x):
  return x+2

syntax="(toggle|turn on|turn off) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)\n"
actions={"toggle": toggle, "turn on": turnon, "turn off": turnoff}
for line in open("6.input"):
  (action, sx, sy, ex, ey)= re.match(syntax, line).groups()
  apply(grid, int(sx), int(sy), int(ex), int(ey), actions[action])
print sum(map(sum, grid))

