#include <iostream>

#include "Eigen/Dense"
#include "ctrl/lls.hpp"

int main() {
    Eigen::MatrixXd x(4, 1);
    Eigen::MatrixXd y(4, 1);

    x << 1, 2, 3, 4;
    y << 1, 11, 21, 32;

    auto solution = ctrl::lls::Solve<2>(
        [](float _x) { return Eigen::RowVector2d{_x, 1}; }, x, y);

    std::cout << solution << std::endl;
    return 0;
}