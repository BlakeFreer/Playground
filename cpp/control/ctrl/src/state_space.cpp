#include "Eigen/Dense"
#include "ctrl/state_space.hpp"

namespace ctrl {

LTISystem::LTISystem(Eigen::MatrixXd A, Eigen::MatrixXd B, Eigen::MatrixXd C)
    : A_(A), B_(B), C_(C) {
    int n = A.rows();
    assert(A.cols() == n);
    assert(B.rows() == n);
    assert(C.cols() == n);
    X_ = Eigen::VectorXd(n);
    X_.setZero();
}

void LTISystem::Initialize(Eigen::VectorXd x) {
    X_ = x;
}

void LTISystem::Update(double u) {
    return Update(Eigen::Vector<double, 1>{u});
}
void LTISystem::Update(const Eigen::VectorXd u) {
    X_ = A_ * X_ + B_ * u;
}

Eigen::VectorXd LTISystem::GetOutput() const {
    return C_ * X_;
}

}  // namespace ctrl