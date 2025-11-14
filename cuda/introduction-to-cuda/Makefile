.PHONY: all
all: add.out add_cuda.out add_cuda_threads.out add_cuda_blocks.out add_cuda_prefetch.out gpu_info.out

add.out: add.cpp
	g++ $< -o $@

%.out: %.cu
	nvcc $< -o $@
