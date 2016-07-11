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
require 'csv'

def parse_file(fn)
  contents = File.read(fn)
  relevant_lines = contents.each_line.find_all {|l| l !~ /^#/ }
  CSV.parse(relevant_lines.map(&:strip).join("\n"), col_sep: "\t")
end

RES_B = 1
RES_W = 2
RES_R = 3

def wins(fn)
  data = parse_file(fn)
  white = data.find_all {|row| row[RES_R] =~ /W\+/ }.count
  black = data.find_all {|row| row[RES_R] =~ /B\+/ }.count
  n = white + black
  p = white.to_f/n
  "#{(p*100).round(2)}% wins (#{white} games of #{n}, ± #{error(p, n, 0.95).round(2)} at 95%, ± #{error(p, n, 0.99).round(2)} at 99%)"
end

def scoring(fn)
  data = parse_file(fn)
  relevant = data.find_all {|row| row[RES_R] !~ /[BW]\+R/ }
  agreeing = relevant.find_all {|row| row[RES_W] == row[RES_B] }.count
  n = relevant.length
  p = agreeing.to_f/n
  "#{(p*100).round(2)}% same score as GnuGo (#{agreeing} of #{n}, ± #{error(p, n, 0.95).round(2)} at 95%, ± #{error(p, n, 0.99).round(2)} at 99%)"
end

def z(confidence)
  alpha = 1 - confidence
  (1 - 0.5*alpha)*2
end

def error(p, n, confidence)
  (z(confidence) * Math.sqrt((1.0/n)*p*(1-p)))*100
end

Dir["*.dat"].each do |fn|
  next if fn =~ /summary\.dat/
  puts "#{fn}:"
  puts "\t\t#{wins(fn)}"
  puts "\t\t#{scoring(fn)}"
end
