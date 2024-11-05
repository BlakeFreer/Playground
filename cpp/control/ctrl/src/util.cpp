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

}  // namespace ctrl