valid = 0
invalid = 0

File.foreach('input.txt') do |line|
	words = line.split(" ")
	if words.length == words.uniq.length
		valid += 1
	else
		puts "invalid: #{words}"
		invalid += 1
	end
end

puts "valid: #{valid}"
puts "invalid: #{invalid}"
