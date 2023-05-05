set terminal png
set title "Reaction of degradation" 
set ylabel "Number of molecules"
set xlabel "Time [s]"
unset key # Uncomment for legend
set grid
set tic scale 0
set yrange [0:20]
set size square
set output 'plots/result.png'
n = system("awk 'NR==1{print NF}' data/data.txt") # Get number of columns
plot for [i=2:n] 'data/data.txt' u 1:i w l title "Realization ".(i-1)

