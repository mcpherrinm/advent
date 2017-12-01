import hashlib

key = "abcdef"
key="iwrupvqb"

for i in xrange(10000000):
  h = hashlib.md5()
  h.update(key + str(i))
  if h.hexdigest().startswith(6*'0'):
    print i
