valid = 0
validwords = 0

File.foreach('input.txt') do |line|
	words = line.split(" ")
	anagrams = words.map do |word|
		word.chars.sort.join
	end
	if anagrams.length == anagrams.uniq.length
		validwords += 1
	end
	if words.length == words.uniq.length
		valid += 1
	end
end

puts "valid: #{valid}"
puts "valid anagrams: #{validwords}"
