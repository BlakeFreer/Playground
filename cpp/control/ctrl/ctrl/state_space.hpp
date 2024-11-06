#pragma once
#include <cassert>

#include "Eigen/Dense"

namespace ctrl {

class LTISystem {
public:
    LTISystem(Eigen::MatrixXd A, Eigen::MatrixXd B, Eigen::MatrixXd C);

    void Initialize(Eigen::VectorXd x);
    void Update(Eigen::VectorXd u);
    void Update(double u);
    Eigen::VectorXd GetOutput() const;

private:
    Eigen::MatrixXd A_;
    Eigen::MatrixXd B_;
    Eigen::MatrixXd C_;
    Eigen::VectorXd X_;
};

}  // namespace ctrl