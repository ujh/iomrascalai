#
# Copyright (c) 2016 Urban Hafner
#
# Permission is hereby granted, free of charge, to any person
# obtaining a copy of this software and associated documentation files
# (the "Software"), to deal in the Software without restriction,
# including without limitation the rights to use, copy, modify, merge,
# publish, distribute, sublicense, and/or sell copies of the Software,
# and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be
# included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
# NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
# BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
# ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#
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
