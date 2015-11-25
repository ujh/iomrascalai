def run(toml:, seed:, outfile:)
  File.open("config.toml", "w") {|f| f.puts toml }
  if rand(2).zero?
    white = iomrascalai
    black = gnugo
  else
    white = gnugo
    black = iomrascalai
  end
  execute(twogtp({
    size: size(seed),
    time: time(seed),
    referee: referee,
    black: black,
    white: white,
    outfile: outfile
  }))
  # Return if we're playing white or not
  white == iomrascalai
end

def execute(command)
  command = "source ~/.bashrc && #{command}"
  $stderr.puts command.inspect
  system command
end

# Select size based on the module of the seed (see Replications in the
# experiments file). For now we just always use 13.
def size(seed)
  13
end

def time(seed)
  case size(seed)
  when 9
    "5m"
  when 13
    "10m"
  when 19
    "20m"
  else
    $stderr.puts "Size #{size(seed)} not supported!"
    exit 1
  end
end

def gnugo
  "gnugo --mode gtp --level 0 --chinese-rules --positional-superko --capture-all-dead --score aftermath --play-out-aftermath"
end

def referee
  gnugo
end

def iomrascalai
  "cargo run --release -- -c config.toml"
end

def twogtp(size:, time:, referee:, black:, white:, outfile:)
  "gogui-twogtp -auto -size #{size} -time #{time} -komi 6.5 -games 1 -verbose -debugtocomment -referee #{referee.inspect} -black #{black.inspect} -white #{white.inspect} -sgffile #{outfile} -force"
end
