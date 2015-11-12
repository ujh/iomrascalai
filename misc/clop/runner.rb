#!/usr/bin/env ruby

require 'fileutils'

processor = ARGV[0] # ignored
SEED = ARGV[1]

def run(command)
  $stderr.puts command.inspect
  system command
end

parameters = {
  play_out_aftermath: 0.0,
  threads: 8.0,
  playout: {
    atari_check: 1.0,
    ladder_check: 1.0,
    last_moves_for_heuristics: 2.0,
    no_self_atari_cutoff: 7.0,
    pattern_probability: 0.9,
    play_in_middle_of_eye: 1.0,
    use_patterns: 1.0,
  },
  priors: {
    best_move_factor: 1.0,
    capture_many: 30.0,
    capture_one: 15.0,
    empty: 20.0,
    neutral_plays: 10.0,
    neutral_wins: 5.0,
    patterns: 10.0,
    self_atari: 10.0,
    use_empty: 1.0,
    use_patterns: 0.0,
  },
  timer: {
    c: 0.5,
  },
  tree: {
    end_of_game_cutoff: 0.08,
    expand_after: 1.0,
    fastplay20_thres: 0.8,
    fastplay5_thres: 0.95,
    rave_equiv: 20.0,
    reuse_subtree: 1.0
  }
}

ARGV[2..-1].each_slice(2) do |name, value|
  level1, level2 = name.split(".").map(&:to_sym)
  value = value.to_f
  parameters[level1][level2] = value
end

def bool(float)
  (float > 0.5).inspect
end

def int(float)
  float.round
end

config = <<EOS
log = true
play_out_aftermath = #{bool(parameters[:play_out_aftermath])}
ruleset = "chinese"
threads = #{int(parameters[:threads])}

[playout]

atari_check = #{bool(parameters[:playout][:atari_check])}
ladder_check = #{bool(parameters[:playout][:ladder_check])}
last_moves_for_heuristics = #{int(parameters[:playout][:last_moves_for_heuristics])}
no_self_atari_cutoff = #{int(parameters[:playout][:no_self_atari_cutoff])}
pattern_probability = #{parameters[:playout][:pattern_probability]}
play_in_middle_of_eye = #{bool(parameters[:playout][:play_in_middle_of_eye])}
use_patterns = #{bool(parameters[:playout][:use_patterns])}

[priors]

best_move_factor = #{parameters[:priors][:best_move_factor]}
capture_many = #{int(parameters[:priors][:capture_many])}
capture_one = #{int(parameters[:priors][:capture_one])}
empty = #{int(parameters[:priors][:empty])}
neutral_plays = #{int(parameters[:priors][:neutral_plays])}
neutral_wins = #{int(parameters[:priors][:neutral_wins])}
patterns = #{int(parameters[:priors][:patterns])}
self_atari = #{int(parameters[:priors][:self_atari])}
use_empty = #{bool(parameters[:priors][:use_empty])}
use_patterns = #{bool(parameters[:priors][:use_patterns])}

[timer]

c = #{parameters[:timer][:c]}

[tree]

end_of_game_cutoff = #{parameters[:tree][:end_of_game_cutoff]}
expand_after = #{int(parameters[:tree][:expand_after])}
fastplay20_thres = #{parameters[:tree][:fastplay20_thres]}
fastplay5_thres = #{parameters[:tree][:fastplay5_thres]}
rave_equiv = #{parameters[:tree][:rave_equiv]}
reuse_subtree = #{bool(parameters[:tree][:reuse_subtree])}
EOS

GNUGO = "gnugo --mode gtp --level 0 --chinese-rules --positional-superko --capture-all-dead --score aftermath --play-out-aftermath"
REFEREE = GNUGO
IOMRASCALAI = "cargo run --release -- -c config.toml"

SIZE=9
TIME="1m"

if rand(2).zero?
  WHITE = GNUGO
  BLACK = IOMRASCALAI
else
  WHITE = IOMRASCALAI
  BLACK = GNUGO
end


TWOGTP = "gogui-twogtp -auto -size #{SIZE} -time #{TIME} -komi 6.5 -games 1 -verbose -debugtocomment -referee #{REFEREE.inspect} -black #{BLACK.inspect} -white #{WHITE.inspect} -sgffile run -force"

dir = File.join(File.dirname(__FILE__), "experiments", SIZE.to_s, SEED.to_s)
FileUtils.mkdir_p(dir)
Dir.chdir(dir) do

  File.open('config.toml', 'w') {|f| f.puts config }

  run(TWOGTP)

  File.open("run.dat", "r") do |f|
    f.each_line do |line|
      next if line =~/^#/
      next if line =~/^\s*$/
      winner = line.split(/\t/)[3]
      won = false
      if WHITE == IOMRASCALAI
        won = winner.include?("W+")
      else
        won = winner.include?("B+")
      end
      if won
        puts "W"
      else
        puts "L"
      end
      exit
    end
  end

end
