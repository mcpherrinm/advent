def move(loc, dir):
  map = {">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0)}
  x=map.get(dir, (0,0))
  return (loc[0]+x[0], loc[1]+x[1])

visited = {}
visited[(0,0)] = 1
where = (0, 0)
robo = (0, 0)

input = open('3.input').read()

for i in range(0, len(input)-1, 2):
  where = move(where, input[i])
  visited[where] = 1
  robo = move(robo, input[i+1])
  visited[robo] = 1

print visited
print len(visited)
