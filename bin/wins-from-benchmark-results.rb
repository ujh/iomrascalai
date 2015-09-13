def data(contents)
  contents.each_line.find_all {|l| l !~ /^#/ }.map do |l|
    l.split(/\s+/)[3]
  end
end

def wins(file)
  contents = File.read(file)
  white = data(contents).find_all {|l| l =~ /W\+/ }.count
  black = data(contents).find_all {|l| l =~ /B\+/ }.count
  percentage = white.to_f/(white+black)
  "#{(percentage*100).round(2)}% wins (#{white} games of #{white+black})"
end

Dir["*.dat"].each do |fn|
  next if fn =~ /summary\.dat/
  puts "#{fn}: #{wins(fn)}"
end
