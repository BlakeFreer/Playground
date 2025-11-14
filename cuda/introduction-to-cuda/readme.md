Following https://developer.nvidia.com/blog/even-easier-introduction-cuda/

Each file adds two arrays of 2^20 floats with increasing GPU usage.

- `add.cpp` serially adds in the CPU

    > Performance not measured

- `add_cuda.cu` serially adds in the GPU (single thread)

    > 63 ms

- `add_cuda_threads.cu` uses 256 threads in the GPU

    > 3.5 ms

- `add_cuda_blocks.cu` uses several blocks of 32 threads in the CPU

    > 2.0 ms

- `add_cuda_prefetch.cu` uses the same block-thread setup as `add_cuda_block.cu` but prefetches the input arrays into the GPU, speeding up computation by reducing memory access time

    > 700 us
