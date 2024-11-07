#include "Eigen/Dense"
#include "ctrl/state_space.hpp"

namespace ctrl {

LTISystem::LTISystem(Eigen::MatrixXd A, Eigen::MatrixXd B, Eigen::MatrixXd C)
    : A_(A), B_(B), C_(C) {
    int nx = GetStateSize();
    assert(A.rows() == nx);
    assert(A.cols() == nx);
    assert(B.rows() == nx);
    assert(C.cols() == nx);

    int ny = GetOutputSize();
    assert(C.rows() == ny);

    X_ = Eigen::VectorXd(nx);
    X_.setZero();
}

int LTISystem::GetStateSize() const {
    return A_.rows();
}
int LTISystem::GetOutputSize() const {
    return C_.rows();
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