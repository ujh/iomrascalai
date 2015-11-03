# -*- coding: utf-8 -*-
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
  mean = (white.to_f/(white+black))*100
  variance = (white * (100-mean) + black * mean) / (n-1)
  sigma = Math.sqrt(variance) / 2
  "#{mean.round(2)}% wins (#{white} games of #{n}, ± #{(2*sigma).round(2)} at 95%, ± #{(3*sigma).round(2)} at 99%)"
end

Dir["*.dat"].each do |fn|
  next if fn =~ /summary\.dat/
  puts "#{fn}: #{wins(fn)}"
end
