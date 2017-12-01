import md5
import sys

map = {}

for n in xrange(1000000000):
   m = md5.new()
   m.update("wtnhxymk")
   m.update(str(n))
   digest = m.hexdigest()
   if digest.startswith("00000"):
     position = digest[5]
     password = digest[6]
     if position not in map.keys():
        map[position] = password
        for pos in xrange(8):
          at = map.get(str(pos))
          if at is not None:
            sys.stdout.write(at)
          else:
            sys.stdout.write("_")
       
        print "  ", map
