set datafile separator ","
set key autotitle columnhead

file="series.out"
plot for [i=2:*] file using 1:i with lines

pause -1