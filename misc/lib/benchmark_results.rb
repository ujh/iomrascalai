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

class BenchmarkResults

  attr_reader :wins, :games, :error95, :error99, :win_percentage, :scoring

  def initialize(filename)
    @filename = filename
    process
  end

  private

  RES_B = 1
  RES_W = 2
  RES_R = 3

  def process
    @data ||= parse_file
    process_win_percentage
    process_score
  end

  def process_win_percentage
    white = @data.find_all {|row| row[RES_R] =~ /W\+/ }.count
    black = @data.find_all {|row| row[RES_R] =~ /B\+/ }.count
    @games = white + black
    @wins = white
    @win_percentage = white.to_f/@games
    @error95 = error(@win_percentage, @games, 0.95)
    @error99 = error(@win_percentage, @games, 0.99)
  end

  def process_score
    @scoring = {}
    relevant = @data.find_all {|row| row[RES_R] !~ /[BW]\+R/ }
    agreeing = relevant.find_all {|row| row[RES_W] == row[RES_B] }.count
    @scoring[:games] = relevant.length
    @scoring[:same_score] = agreeing
    @scoring[:same_score_percentage] = agreeing.to_f/@scoring[:games]
    p = agreeing.to_f/@scoring[:games]
    @scoring[:error95] = error(p, @scoring[:games], 0.95)
    @scoring[:error99] = error(p, @scoring[:games], 0.99)
  end

  def parse_file
    contents = File.read(@filename)
    relevant_lines = contents.each_line.find_all {|l| l !~ /^#/ }
    CSV.parse(relevant_lines.map(&:strip).join("\n"), col_sep: "\t")
  end

  def z(confidence)
    alpha = 1 - confidence
    (1 - 0.5*alpha)*2
  end

  def error(p, n, confidence)
    (z(confidence) * Math.sqrt((1.0/n)*p*(1-p)))*100
  end

end
