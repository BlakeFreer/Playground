.PHONY: all
all: add add_cuda add_cuda_threads add_cuda_blocks add_cuda_prefetch

add: add.cpp
	g++ $< -o $@

%: %.cu
	nvcc $< -o $@
