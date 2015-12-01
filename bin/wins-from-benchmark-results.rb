# -*- coding: utf-8 -*-
#
# Copyright (c) 2015 Urban Hafner
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

def data(contents)
  contents.each_line.find_all {|l| l !~ /^#/ }.map do |l|
    l.split(/\s+/)[3]
  end
end

def wins(file)
  contents = File.read(file)
  white = data(contents).find_all {|l| l =~ /W\+/ }.count
  black = data(contents).find_all {|l| l =~ /B\+/ }.count
  n = white + black
  p = white.to_f/n
  "#{(p*100).round(2)}% wins (#{white} games of #{n}, ± #{error(p: p, n: n, confidence: 0.95).round(2)} at 95%, ± #{error(p: p, n: n, confidence: 0.99).round(2)} at 99%)"
end

def z(confidence:)
  alpha = 1 - confidence
  (1 - 0.5*alpha)*2
end

def error(p:, n:, confidence:)
  (z(confidence: confidence) * Math.sqrt((1.0/n)*p*(1-p)))*100
end

Dir["*.dat"].each do |fn|
  next if fn =~ /summary\.dat/
  puts "#{fn}: #{wins(fn)}"
end
