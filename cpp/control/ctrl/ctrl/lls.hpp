#pragma once

#include "Eigen/Dense"

namespace ctrl::lls {

auto Solve(std::function<Eigen::RowVectorXd(float)> create_row,
           const Eigen::VectorXd& x, const Eigen::VectorXd& y)
    -> Eigen::VectorXd {
    // Construct A matrix
    int n_rows = x.rows();
    int n_param = create_row(0).cols();
    Eigen::MatrixXd A(n_rows, n_param);

    for (int i = 0; i < n_rows; i++) {
        A.row(i) = create_row(x(i));
    }

    // Solve the normal equation
    auto ata = (A.transpose() * A);
    return ata.inverse() * A.transpose() * y;
}

}  // namespace ctrl::lls