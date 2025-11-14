#include <cuda_runtime.h>

#include <iostream>

int main() {
    int deviceCount;
    cudaGetDeviceCount(&deviceCount);

    for (int i = 0; i < deviceCount; ++i) {
        cudaDeviceProp deviceProp;
        cudaGetDeviceProperties(&deviceProp, i);

        std::cout << "--- Device " << i << " ---" << std::endl;
        std::cout << "Name: " << deviceProp.name << std::endl;
        std::cout
            << "CUDA Cores: "
            << deviceProp.multiProcessorCount *
                   ((deviceProp.major == 9)
                        ? 128
                        : ((deviceProp.major >= 3)
                               ? (deviceProp.major == 8 &&
                                  deviceProp.minor == 6)
                                     ? 128
                                     : ((deviceProp.major == 8 &&
                                         deviceProp.minor == 0)
                                            ? 64
                                            : ((deviceProp.major == 7)
                                                   ? 64
                                                   : ((deviceProp.major == 6 &&
                                                       deviceProp.minor == 1)
                                                          ? 128
                                                          : ((deviceProp
                                                                      .major ==
                                                                  6 &&
                                                              deviceProp
                                                                      .minor ==
                                                                  0)
                                                                 ? 64
                                                                 : ((deviceProp
                                                                         .major ==
                                                                     5)
                                                                        ? 128
                                                                        : ((deviceProp
                                                                                .major ==
                                                                            3)
                                                                               ? 192
                                                                               : 0))))))
                               : 0))
            << std::endl;  // Simplified core count calculation
        std::cout << "Total Global Memory: "
                  << deviceProp.totalGlobalMem / (1024 * 1024) << " MB"
                  << std::endl;
        std::cout << "Compute Capability: " << deviceProp.major << "."
                  << deviceProp.minor << std::endl;
        // Add more properties as needed
    }
    return 0;
}
