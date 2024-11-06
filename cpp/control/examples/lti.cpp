#include <format>
#include <iostream>

#include "Eigen/Dense"
#include "ctrl/state_space.hpp"

void print(double t, ctrl::LTISystem sys, double input) {
    std::string output = std::format("{:.4f}", t);
    auto y = sys.GetOutput();
    for (auto x : y) {
        output += std::format(",{:.4f}", x);
    }
    output += std::format(",{:.4f}", input);
    std::cout << output << std::endl;
}

void kill(int code, std::string msg) {
    std::cerr << msg << std::endl;
    exit(code);
}

double parse_double(char* arg) {
    char* end;
    double x = std::strtod(arg, &end);
    if (end == arg) kill(2, "Invalid double.");
    return x;
}

struct Config {
    double drag;
    double timestep;
};

Config parse_args(int argc, char* argv[]) {
    if (argc != 3) kill(2, "Usage: ./lti <drag> <timestep>");

    Config c;
    c.drag = parse_double(argv[1]);
    c.timestep = parse_double(argv[2]);

    if (c.drag < 0) kill(2, "<drag> must be positive");
    if (c.timestep < 0) kill(2, "<timestep> must be positive.");

    return c;
}

int main(int argc, char* argv[]) {
    Config c = parse_args(argc, argv);

    // Model a car with drag.
    Eigen::MatrixXd A(2, 2);
    Eigen::MatrixXd B(2, 1);
    Eigen::MatrixXd C(2, 2);

    A << 1, c.timestep, 0, 1 - c.drag * c.timestep;
    B << 0, c.timestep;
    C.setIdentity();  // output is full state [p, v]

    ctrl::LTISystem sys{A, B, C};

    std::cout << "Time,Position,Velocity,Acceleration" << std::endl;

    double time = 0;

    // gas
    double accel = 1;
    print(time, sys, accel);

    int gas_time = 5;
    for (time = c.timestep; time < gas_time; time += c.timestep) {
        sys.Update(accel);
        print(time, sys, accel);
    }

    // Let come to rest
    accel = 0;
    double stopped_limit = 0.01;
    for (; sys.GetOutput()[1] > stopped_limit; time += c.timestep) {
        sys.Update(accel);
        print(time, sys, accel);
    }

    return 0;
}