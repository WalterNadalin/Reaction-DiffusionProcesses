datafile = 'data/data.txt'

stats datafile nooutput
n = STATS_blocks - 1

set terminal png
set title sprintf("%d independent reaction(s) of degradation", n)
set ylabel "Number of molecules"
set xlabel "Time [s]"
unset key
set grid
set tic scale 0
set size square
set output 'plots/result.png'

plot for [i=0:(n-1)] datafile u 1:2 index i w l
