#pragma once

#include "Eigen/Dense"

namespace ctrl {

/**
 * @param initial Initial parameters.
 * @param gradient_function Gradient of the loss function given parameters.
 * @param learning_rate How fast to move, relative to the gradient.
 * @param convergence Stop iterating when `||x(i+1) - x(i)||` < convergence
 * @return Eigen::VectorXd Final parameters
 */
Eigen::VectorXd SolveGradientDescent(
    const Eigen::VectorXd initial,
    std::function<Eigen::VectorXd(Eigen::VectorXd)> gradient_function,
    double learning_rate, double convergence);

}  // namespace ctrl