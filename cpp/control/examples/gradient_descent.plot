# Usage:
# Compile and run `examples/gradient_descent` and pipe the output to a file like `out.txt`.
# Then plot with
# gnuplot -e "file='out.txt'" gradient_descent.plot

set datafile separator ","
set key autotitle columnhead

set style lines 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1

set style lines 2 \
    linecolor rgb '#dd181f' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1.5

plot file using 1:2 with points linestyle 1, \
     file using 1:3 with lines linestyle 2

pause -1