string=0
memory=0
for line in open("8.in"):
  memory+=eval("len(" + line.rstrip() + ")")
  string+=len(line.rstrip())
print string-memory
