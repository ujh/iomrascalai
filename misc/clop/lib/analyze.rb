def analyze(outfile:, playing_white:)
  File.open("#{outfile}.dat", "r") do |f|
    f.each_line do |line|
      next if line =~/^#/
      next if line =~/^\s*$/
      winner = line.split(/\t/)[3]
      won = false
      if playing_white
        won = winner.include?("W+")
      else
        won = winner.include?("B+")
      end
      if won
        return "W"
      else
        return "L"
      end
    end
  end
end
