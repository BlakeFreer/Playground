#include <grpcpp/grpcpp.h>
#include <iostream>
#include "proto/greeter.pb.h"

int main() {
    std::cout << "gRPC version: " << grpc::Version() << std::endl;

    HelloRequest h;
    h.set_name("Blake");

    std::cout << h.name() << std::endl;

    return 0;
}
