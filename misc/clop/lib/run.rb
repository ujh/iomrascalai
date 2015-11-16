def run(toml:, seed:, outfile:)
  File.open("config.toml", "w") {|f| f.puts toml }
  execute(twogtp({
    size: size(seed),
    time: time(seed),
    referee: referee,
    black: gnugo,
    white: iomrascalai,
    outfile: outfile
  }))
  # Just return true for now as we always play white
  true
end

def execute(command)
  command = "source ~/.bashrc && #{command}"
  $stderr.puts command.inspect
  system command
end

def size(seed)
  (seed % 2).zero? ? 9 : 13
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
