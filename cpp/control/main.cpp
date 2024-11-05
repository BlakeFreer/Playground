#include <iostream>
#include <tuple>

#include "Eigen/Dense"

int main() {
    Eigen::MatrixXd A(2, 2);
    A << 1, 2, 3, 4;

    std::cout << "Here is the matrix A:\n" << A << std::endl;

    return 0;
}