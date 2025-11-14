#include <math.h>

#include <iostream>

__global__ void add(int n, float* x, float* y) {
    int start = threadIdx.x;
    int stride = blockDim.x;
    for (int i = start; i < n; i += stride) {
        y[i] = x[i] + y[i];
    }
}

int main(void) {
    int n = 1 << 20;  // 1 million elements

    float* x = new float[n];
    float* y = new float[n];

    // Allocate unified memory - accessible from host (cpu) or device (gpu)
    cudaMallocManaged(&x, n * sizeof(float));
    cudaMallocManaged(&y, n * sizeof(float));

    // initialize x and y arrays on the host (CPU)

    for (int i = 0; i < n; i++) {
        x[i] = 1.0f;
        y[i] = 2.0f;
    }

    // run the kernel on the GPU
    add<<<1, 256>>>(n, x, y);

    // wait for it to finish
    cudaDeviceSynchronize();

    // assert all values are near 3
    float max_error = 0.0;
    for (int i = 0; i < n; i++) {
        max_error = fmax(max_error, y[i] - 3.0f);
    }

    std::cout << "Max error: " << max_error << std::endl;

    // free memory
    cudaFree(x);
    cudaFree(y);

    return 0;
}
