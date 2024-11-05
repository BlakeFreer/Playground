/// Simple LLS solver with gradient descent.

#include <format>
#include <iostream>
#include <random>

#include "Eigen/Dense"
#include "ctrl/ctrl.hpp"

Eigen::RowVectorXd model(double x) {
    return Eigen::RowVector3d{1, x, x * x};
}

Eigen::VectorXd gradient(const Eigen::VectorXd& x, const Eigen::VectorXd& y,
                         const Eigen::VectorXd& param) {
    Eigen::VectorXd gradient(param.size());
    gradient.setZero();

    for (int i = 0; i < x.size(); i++) {
        auto m = model(x[i]).transpose();
        gradient += 2 * (model(x[i]).dot(param) - y[i]) * m;
    }

    return gradient;
}

int main() {
    // Create noisy data from a model.
    std::default_random_engine rng;
    std::normal_distribution<double> noise(0, 1);

    const int N = 200;
    Eigen::Vector3d param{1, -2, 3};

    Eigen::VectorXd x = ctrl::linspace(-5, 5, N, true);
    Eigen::VectorXd y =
        x.unaryExpr([&](float i) { return model(i).dot(param); });

    for (int i = 0; i < N; i++) {
        y[i] += noise(rng);
    }

    // Solve with gradient descent.
    auto param_gd = ctrl::SolveGradientDescent(
        Eigen::VectorXd::Zero(param.size()),
        [&](auto p) { return gradient(x, y, p); }, 0.001 / N, 00001);

    // Print results to stdout so they can be piped to a file.
    std::cout << "x, raw, fit\n";
    for (int i = 0; i < N; i++) {
        std::cout << std::format("{:.4f}, {:.4f}, {:.4f}\n", x[i], y[i],
                                 model(x[i]).dot(param_gd));
    }

    std::cerr << "True parameters: " << param.transpose() << "\n";
    std::cerr << "Solved parameters: " << param_gd.transpose() << "\n";

    return 0;
}