# -*- coding: utf-8 -*-
#
# Copyright (c) 2015 Urban Hafner
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
require_relative "../misc/lib/benchmark_results"

files = Dir["*.dat"].sort do |a, b|
  a =~ /(.*)-(\d+)x\d+/
  prefix_a = $1
  size_a = $2.to_i
  b =~ /(.*)-(\d+)x\d+/
  prefix_b = $1
  size_b = $2.to_i
  prefix_comparision = prefix_a <=> prefix_b
  if prefix_comparision.zero?
    size_a <=> size_b
  else
    prefix_comparision
  end
end

files.each do |fn|
  next if fn =~ /summary\.dat/
  puts "#{fn}:"
  br = BenchmarkResults.new(fn)
  puts "\t\t#{(br.win_percentage*100).round(2)}% wins (#{br.wins} games of #{br.games}, ± #{br.error95.round(2)} at 95%, ± #{br.error99.round(2)} at 99%)"

  scoring = br.scoring
  puts "\t\t#{(scoring[:same_score_percentage]*100).round(2)}% same score as GnuGo (#{scoring[:same_score]} of #{scoring[:games]}, ± #{scoring[:error95].round(2)} at 95%, ± #{scoring[:error99].round(2)} at 99%)"
end
