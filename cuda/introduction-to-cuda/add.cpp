#include <math.h>

#include <iostream>

void add(int n, float* x, float* y) {
    for (int i = 0; i < n; i++) {
        y[i] = x[i] + y[i];
    }
}

int main(void) {
    int n = 1 << 20;  // 1 million elements

    float* x = new float[n];
    float* y = new float[n];

    // initialize x and y arrays on the host (CPU)

    for (int i = 0; i < n; i++) {
        x[i] = 1.0f;
        y[i] = 2.0f;
    }

    // run the "kernel" on the CPU

    add(n, x, y);

    // assert all values are near 3
    float max_error = 0.0;
    for (int i = 0; i < n; i++) {
        max_error = fmax(max_error, y[i] - 3.0f);
    }

    std::cout << "Max error: " << max_error << std::endl;

    // free memory
    delete[] x;
    delete[] y;

    return 0;
}
