#include <format>
#include <iostream>

#include "Eigen/Dense"
#include "ctrl/ctrl.hpp"

double accel_fcn(double t) {
    if (t < 5) {
        return 1;
    } else {
        return 0;
    }
}

struct Config {
    double timestep;
    double noise;
};

void kill(std::string msg, int err) {
    std::cerr << msg << std::endl;
    exit(err);
}
double parse_double(char* str) {
    char* s = str;
    double d = std::strtod(str, &s);
    if (s == str) kill(std::format("Invalid double \"{}\".", str), 2);
    return d;
}
Config parse(int argc, char* argv[]) {
    if (argc != 3)
        kill(std::format("Usage: ./kalman <timestep> <noise>", argv[0]), 2);

    Config c = {
        .timestep = parse_double(argv[1]),
        .noise = parse_double(argv[2]),
    };

    if (c.timestep <= 0) kill("<timestep> must be positive.", 2);
    if (c.noise <= 0) kill("<noise> must be positive.", 2);
    return c;
}

int main(int argc, char* argv[]) {
    Config c = parse(argc, argv);

    Eigen::MatrixXd A(2, 2);
    Eigen::MatrixXd B(2, 1);
    Eigen::MatrixXd C(1, 2);
    Eigen::VectorXd q(2);
    Eigen::VectorXd r(1);

    const double drag = 0.5;

    A << 1, c.timestep, 0, 1 - drag * c.timestep;
    B << 0, c.timestep;
    C << 1, 0;
    q << 1, 1;
    r << c.noise;

    ctrl::KalmanPlant plant{ctrl::LTISystem{A, B, C}, q * 0, r};
    ctrl::KalmanModel model{ctrl::LTISystem{A, B, C}, q * c.timestep, r};

    model.Initialize(Eigen::MatrixXd::Zero(2, 1),
                     Eigen::MatrixXd::Identity(2, 2));

    std::cout << "time,input,actual,measured,estimate\n";

    double time = 0;
    double input = 1;

    // capture (time, input) by ref to track their changes
    auto print = [&](ctrl::KalmanPlant p, Eigen::VectorXd y,
                     ctrl::KalmanModel m) {
        std::cout << std::format("{:.4f},{:.4f},{:.4f},{:.4f},{:.4f}\n", time,
                                 input, p.GetOutput()[0], y[0],
                                 m.GetStateEstimate()[0]);
    };

    // accelerate
    for (; time < 10; time += c.timestep) {
        input = accel_fcn(time);

        plant.Update(input);
        model.Update(input);

        auto measured = plant.Measure();
        model.Predict(measured);

        print(plant, measured, model);
    }

    return 0;
}