# Runs genmove for 9x9, 13x13, and 19x19 from 1 upto N threads and outputs the data to STDOUT in CSV
# format. Can be used to find the optimal number of threads to use for a given machine.
#
# Takes the max. number of threads to calculate as the first argument.

require 'open3'

THREADS = ARGV[0] ? ARGV[0].to_i : 36

INPUT = "boardsize 9\nclear_board\ntime_settings 300 0 0\ngenmove b\nboardsize 13\nclear_board\ntime_settings 600 0 0\ngenmove b\nboardsize 19\nclear_board\ntime_settings 1200 0 0\ngenmove b\n"

def run(threads)
  cfg = "config.toml"
  File.open(cfg, "w") do |f|
    f.puts "threads = #{threads}"
  end
  cmd = "cargo run --release -- -l -c #{cfg}"
  stderr = Open3.popen3(cmd) {|i,o,e,t| i.puts(INPUT); i.close; e.read }
  stderr.split($/).find_all do |l|
    l =~/pps per thread/
  end.map do |l|
    l =~/(\d+)pps \((\d+)pps per thread/;
    [$1, $2]
  end
end

stats = []
1.upto(THREADS) do |threads|
  p threads
  thread_stats = [threads] + run(threads)
  stats << thread_stats
end

puts "threads, 9x9 all, 9x9 per thread, 13x13 all, 13x13 per thread, 19x19 all, 19x19 per thread"
stats.each do |ts|
  puts ts.join(", ")
end
