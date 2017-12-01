def paperfor(l, w, h):
  sides = sorted([l*w, w*h, h*l])
  return 3*sides[0] + 2*sides[1] + 2*sides[2]
  
assert paperfor(2,3,4) == 58
assert paperfor(1,1,10) == 43

def ribbonfor(l,w,h):

f = open("2.input").readlines()
sum = 0
for line in f:
  pkg = line.rstrip().split("x")
  sum += paperfor(int(pkg[0]), int(pkg[1]), int(pkg[2]))

print sum
