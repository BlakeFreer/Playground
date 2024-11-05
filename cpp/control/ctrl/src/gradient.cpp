#include "ctrl/gradient.hpp"

namespace ctrl {

auto SolveGradientDescent(
    Eigen::VectorXd initial,
    std::function<Eigen::VectorXd(Eigen::VectorXd)> gradient_function,
    double learning_rate, double convergence) -> Eigen::VectorXd {
    auto param = Eigen::VectorXd{initial};
    Eigen::VectorXd gradient;

    do {
        gradient = gradient_function(param);
        param -= gradient * learning_rate;
    } while (gradient.norm() > convergence);
    return param;
}

}  // namespace ctrl