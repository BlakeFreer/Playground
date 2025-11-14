#include <cuda_runtime.h>

#include <iostream>

int main() {
    int deviceCount;
    cudaGetDeviceCount(&deviceCount);

    for (int i = 0; i < deviceCount; ++i) {
        cudaDeviceProp prop;
        cudaGetDeviceProperties(&prop, i);

        std::cout << "--- Device " << i << " ---" << std::endl;
        std::cout << "Name: " << prop.name << std::endl;
        std::cout << "CUDA MultiProcessors: " << prop.multiProcessorCount
                  << std::endl;
        std::cout << "Total Global Memory: "
                  << prop.totalGlobalMem / (1024 * 1024) << " MB" << std::endl;
        std::cout << "Device Location: "
                  << (prop.integrated ? "Integrated" : "Discrete") << std::endl;
        std::cout << "Compute Capability: " << prop.major << "." << prop.minor
                  << std::endl;
        std::cout << "Max Threads per Block: " << prop.maxThreadsPerBlock
                  << std::endl;
        std::cout << "Max Threads per Block Dimension: ("
                  << prop.maxThreadsDim[0] << ", " << prop.maxThreadsDim[1]
                  << ", " << prop.maxThreadsDim[2] << ")" << std::endl;
        std::cout << "Max Threads per MultiProcessor: "
                  << prop.maxThreadsPerMultiProcessor << std::endl;
        std::cout << "Registers per MultiProcessor: "
                  << prop.regsPerMultiprocessor << std::endl;
        std::cout << "Shared Memory per Block: "
                  << prop.sharedMemPerBlock / 1024 << " kB" << std::endl;
        std::cout << "Shared Memory per MultiProcessor: "
                  << prop.sharedMemPerMultiprocessor / 1024 << " kB"
                  << std::endl;
        std::cout << "Threads per Warp: " << prop.warpSize << std::endl;
    }
    return 0;
}
