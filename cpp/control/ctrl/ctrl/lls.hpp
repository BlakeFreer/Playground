#pragma once

#include "Eigen/Dense"

namespace ctrl::lls {

template <int N>
auto Solve(std::function<Eigen::RowVector<double, N>(float)> create_row,
           Eigen::VectorXd x, Eigen::VectorXd y)
    -> Eigen::Matrix<double, N, 1> {
    // Construct A matrix
    int n_rows = x.rows();
    Eigen::MatrixXd A(n_rows, N);

    for (int i = 0; i < n_rows; i++) {
        A.row(i) = create_row(x(i));
    }

    // Solve the normal equation
    auto ata = (A.transpose() * A);
    return ata.inverse() * A.transpose() * y;
}

}  // namespace ctrl::lls