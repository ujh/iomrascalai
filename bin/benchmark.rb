#!/usr/bin/env ruby
# coding: utf-8
#
# Copyright (c) 2016 Urban Hafner
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

require 'csv'
require 'fileutils'
require 'optparse'

OptionParser.new do |opts|
  # Use this when running on a 36 core machine. Then gogui-twogtp will start 4 games in parallel and
  # Iomrascálaí will use 8 threads instead of all 36.
  opts.on("--ec2") do |v|
    $ec2 = true
  end
end.parse!

SIZE = ARGV[0]
PREFIX = ARGV[1] || `git rev-parse --short HEAD`.chop
FILENAME = "#{PREFIX}-#{SIZE}x#{SIZE}"
GNUGO="gnugo --mode gtp --level 0 --chinese-rules --positional-superko"
REFEREE="#{GNUGO}"


if $ec2
  IOMRASCALAI="cargo run --release -- --log --rules chinese --threads 8"
else
  IOMRASCALAI="cargo run --release -- --log --rules chinese"
end

case SIZE
when "9"
  TIME = "2m"
  GAMES = "500"
when "13"
  TIME = "10m"
  GAMES = "1000"
when "19"
  TIME = "20m"
  GAMES = "1000"
else
  raise "Size #{SIZE} isn't supported!"
end

GOGUI_TWOGTP = %Q/gogui-twogtp -auto -black "#{GNUGO}" -white "#{IOMRASCALAI}" -size #{SIZE} -alternate -games #{GAMES} -sgffile #{FILENAME} -time #{TIME} -referee "#{REFEREE}" -verbose/
GOGUI_TWOGTP += " -threads 4" if $ec2

DAT_FILE = "#{FILENAME}.dat"

def run(cmd)
  puts cmd
  system cmd
end

def run_benchmark
  run(GOGUI_TWOGTP)
end

def data
  if File.exists?(DAT_FILE)
    File.read(DAT_FILE)
  else
    ""
  end
end

def parse_file
  relevant_lines = data.each_line.find_all {|l| l !~ /^#/ }
  header = data.each_line.find_all {|l| l =~ /^#/ }.join("")
  [header, CSV.parse(relevant_lines.map(&:strip).join("\n"), col_sep: "\t")]
end

GNUGO_ERROR = /The Go program terminated unexpectedly/
ERR_MSG = 12
GAME_ID = 0
RES_B = 1

def error_found?
  _, contents = parse_file
  contents.any? do |row|
    # Only remove GnuGo errors. Its results are always recorded as black.
    row[RES_B] == "?" && row[ERR_MSG] =~ GNUGO_ERROR
  end
end

def id_of_first_error(contents)
  row = contents.find do |row|
    # Only remove GnuGo errors. Its results are always recorded as black.
    row[RES_B] == "?" && row[ERR_MSG] =~ GNUGO_ERROR
  end
  row[GAME_ID]
end

def remove_sgf_file(error_id)
  sgf_name = "#{FILENAME}-#{error_id}.sgf"
  File.unlink(sgf_name) if File.exists?(sgf_name)
end

def shift_sgf_files(error_id, contents)
  to_shift = contents.find_all {|row| row[GAME_ID].to_i > error_id.to_i }
  to_shift.each do |row|
    old_name = "#{FILENAME}-#{row[GAME_ID]}.sgf"
    next unless File.exists?(old_name)
    new_name = "#{FILENAME}-#{row[GAME_ID].to_i-1}.sgf"
    FileUtils.mv(old_name, new_name)
  end
end

def shift_game_ids(error_id, contents)
  contents.map do |row|
    new_row = row.dup
    new_row[GAME_ID] = "#{new_row[GAME_ID].to_i-1}" if row[GAME_ID].to_i > error_id.to_i
    new_row
  end
end

def remove_first_error
  header, contents = parse_file
  error_id = id_of_first_error(contents)
  # remove SGF of errored game
  remove_sgf_file(error_id)
  # shift following SGFs down
  shift_sgf_files(error_id, contents)
  # remove row of errored game
  contents = contents.find_all {|row| row[GAME_ID] != error_id }
  # shift following rows down (change GAME IDs)
  contents = shift_game_ids(error_id, contents)
  # save updated file to disk
  FileUtils.mv(DAT_FILE, "#{DAT_FILE}-#{Time.now.to_f}")
  File.open(DAT_FILE, 'w') {|f| f.write header }
  CSV.open(DAT_FILE, 'a', col_sep: "\t") {|csv| contents.each {|row| csv << row }}
end

def check_for_crashes
  return unless File.exists?(DAT_FILE)
  # Remove each error one by one
  loop do
    break unless error_found?
    remove_first_error
  end
end

def done?
  return true unless File.exists?(DAT_FILE)
  _, contents = parse_file
  contents.length == GAMES.to_i
end

loop do
  check_for_crashes
  run_benchmark
  check_for_crashes
  break if done?
end
