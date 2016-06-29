# Convert pachi/michi pattern files into one file that only contains the pattern
# and the probability.
require 'csv'
patterns = Hash.new {|hash, key| hash[key] = {} }

CSV.foreach('patterns.prob', col_sep: ' ') do |row|
  probability = row[0].to_f
  next if probability.zero?
  row[3] =~ /\(s:(\d+)\)/
  id = $1
  patterns[id][:probability] = probability
end

File.open('patterns.spat') do |f|
  f.each_line do |line|
    next if line =~ /^#/
    row = CSV.parse(line, col_sep: ' ').flatten
    id = row[0]
    pattern = row[2]
    patterns[id][:pattern] = pattern
  end
end

CSV.open('patterns.input', 'w', col_sep: ' ') do |csv|
  patterns.each_value do |pattern|
    if pattern[:probability] && pattern[:pattern]
      csv << [pattern[:probability], pattern[:pattern]]
    end
  end
end
