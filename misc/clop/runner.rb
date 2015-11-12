require 'fileutils'

$stderr.puts ARGV.inspect

processor = ARGV[0] # ignored
SEED = ARGV[1]

def run(command)
  $stderr.puts command.inspect
  system command
end

parameters = {
  play_out_aftermath: 0,
  threads: 8,
  playout: {
    atari_check: 1, # boolean
    ladder_check: 1, # boolean
    last_moves_for_heuristics: 2,
    no_self_atari_cutoff: 7,
    pattern_probability: 0.9,
    play_in_middle_of_eye: 1, # boolean
    use_patterns: 1, # boolean
  },
  priors: {
    best_move_factor: 1.0,
    capture_many: 30,
    capture_one: 15,
    empty: 20,
    neutral_plays: 10,
    neutral_wins: 5,
    patterns: 10,
    self_atari: 10,
    use_empty: 1, # boolean
    use_patterns: 0, # boolean
  },
  timer: {
    c: 0.5,
  },
  tree: {
    end_of_game_cutoff: 0.08,
    expand_after: 1,
    fastplay20_thres: 0.8,
    fastplay5_thres: 0.95,
    rave_equiv: 20.0,
  }
}

ARGV[2..-1].each_slice(2) do |name, value|
  level1, level2 = name.split(".").map(&:to_sym)
  value = value.to_f
  parameters[level1][level2] = value
end

def bool(number)
  number.round.zero?.inspect
end

config = <<EOS
play_out_aftermath = #{bool(parameters[:play_out_aftermath])}
threads = #{parameters[:threads]}

[playout]

atari_check = #{bool(parameters[:playout][:atari_check])}
ladder_check = #{bool(parameters[:playout][:ladder_check])}
last_moves_for_heuristics = #{parameters[:playout][:last_moves_for_heuristics]}
no_self_atari_cutoff = #{parameters[:playout][:no_self_atari_cutoff]}
pattern_probability = #{parameters[:playout][:pattern_probability]}
play_in_middle_of_eye = #{bool(parameters[:playout][:play_in_middle_of_eye])}
use_patterns = #{bool(parameters[:playout][:use_patterns])}

[priors]

best_move_factor = #{parameters[:priors][:best_move_factor]}
capture_many = #{parameters[:priors][:capture_many]}
capture_one = #{parameters[:priors][:capture_one]}
empty = #{parameters[:priors][:empty]}
neutral_plays = #{parameters[:priors][:neutral_plays]}
neutral_wins = #{parameters[:priors][:neutral_wins]}
patterns = #{parameters[:priors][:patterns]}
self_atari = #{parameters[:priors][:self_atari]}
use_empty = #{bool(parameters[:priors][:use_empty])}
use_patterns = #{bool(parameters[:priors][:use_patterns])}

[timer]

c = #{parameters[:timer][:c]}

[tree]

end_of_game_cutoff = #{parameters[:tree][:end_of_game_cutoff]}
expand_after = #{parameters[:tree][:expand_after]}
fastplay20_thres = #{parameters[:tree][:fastplay20_thres]}
fastplay5_thres = #{parameters[:tree][:fastplay5_thres]}
rave_equiv = #{parameters[:tree][:rave_equiv]}
EOS

GNUGO = "gnugo --mode gtp --level 0 --chinese-rules --positional-superko --capture-all-dead --score aftermath --play-out-aftermath"
REFEREE = GNUGO
IOMRASCALAI = "cargo run --release -- -c config.toml"

case SIZE
when 9
  TIME = "5m"
when 13
  TIME = "10m"
when 19
  TIME = "20m"
else
  puts "Size #{SIZE} not supported!"
  exit 1
end

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
