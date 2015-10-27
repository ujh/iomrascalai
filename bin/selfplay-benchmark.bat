@echo on
REM configuration
set threads=8
set size=13

REM set up the engines
set iomrascalai=..\target\release\iomrascalai.exe -t %threads% --play-out-aftermath true

REM set up which color each engine plays
set black=%iomrascalai%
set white=%iomrascalai%

REM set up the script to run both engines against each other, change the path to the gogui-gtp executable if it's different for you
set twogtp="C:\Program Files (x86)\GoGui\gogui-twogtp"
set test=%twogtp% -black "%black%" -white "%white%" -verbose -size %size%  -time 10m -games 100 -sgffile ../selfplay-benchmark -auto -komi 7.5

%test%