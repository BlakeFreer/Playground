#include <random>

#include "Eigen/Dense"
#include "ctrl/util.hpp"

namespace ctrl {

Eigen::VectorXd linspace(double start, double stop, int num_points,
                         bool endpoint) {
    Eigen::VectorXd x{num_points};
    double step;

    if (endpoint) {
        step = (stop - start) / (num_points - 1);
    } else {
        step = (stop - start) / num_points;
    }

    for (int i = 0; i < num_points; i++) {
        x[i] = start + step * i;
    }

    return x;
}

Eigen::MatrixXd random_from_variance(const Eigen::MatrixXd& variance) {
    static std::random_device generator;
    static std::normal_distribution<double> d(0, 1);
    return variance.unaryExpr([](double v) { return sqrt(v) * d(generator); });
}

}  // namespace ctrl