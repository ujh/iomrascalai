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
