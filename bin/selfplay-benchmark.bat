@echo off
REM configuration
set threads=1
set size=13

REM set up the engines, use random for the fastest possible test
set iomrascalai=..\target\release\iomrascalai.exe -t %threads% --play-out-aftermath true
set random=%iomrascalai% -e random

REM set up which color each engine plays
set black=%iomrascalai%
set white=%random%

REM set up the script to run both engines against each other, change the path to the gogui-gtp executable if it's different for you
set twogtp="C:\Program Files (x86)\GoGui\gogui-twogtp"
set test=%twogtp% -black "%black%" -white "%white%" -verbose -size %size%  -time 10m -games 100 -sgffile ../selfplay-benchmark -auto -komi 7.5

%test%