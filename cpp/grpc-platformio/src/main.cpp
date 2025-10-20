#include <grpcpp/grpcpp.h>
#include <iostream>

int main() {
    std::cout << "gRPC version: " << grpc::Version() << std::endl;
    return 0;
}
