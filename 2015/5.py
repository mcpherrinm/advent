import re
def threevowels(s):

  count = 0
  vowels = "aeiou"
  for c in s:
    if c in vowels:
       count += 1
  return count >= 3

assert threevowels("aei")
assert threevowels("xazegov")
assert not threevowels("hi")

def doubly(s):
  for i in xrange(1,len(s)):
    if s[i] == s[i-1]:
      return True
  return False

assert doubly("abb")
assert doubly("aab")
assert not doubly("abc")

def unblacklist(s):
  for i in ["ab", "cd", "pq", "xy"]:
    if i in s:
      return False
  return True

assert unblacklist("ugknbfddgicrmopn")
assert not unblacklist("asdfxyasdf")



def nice(s):
  return threevowels(s) and doubly(s) and unblacklist(s)

assert nice("ugknbfddgicrmopn")
assert nice("aaa")
assert not nice("jchzalrnumimnmhp")
assert not nice("haegwjzuvuyypxyu")
assert not nice("dvszwmarrgswjxmb")

def newnice(s):
  a,b= (re.match(r".*(..).*\1", s), re.match(r".*(.).\1", s))
  return a and b

count = 0
for line in open("5.input"):
  if newnice(line):
    count += 1

print count

