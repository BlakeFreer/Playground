
#include <iostream>
#include <random>

#include "Eigen/Dense"
#include "ctrl/kalman.hpp"
#include "ctrl/state_space.hpp"
#include "ctrl/util.hpp"

static Eigen::MatrixXd SquareDiagonal(const Eigen::VectorXd& x) {
    Eigen::MatrixXd I{x.size(), x.size()};
    I.setIdentity();
    return (x * x.transpose()).cwiseProduct(I);
}

namespace ctrl {

KalmanPlant::KalmanPlant(LTISystem sys, Eigen::VectorXd q, Eigen::VectorXd r)
    : LTISystem(sys), q_(q), r_(r) {
    assert(q_.size() == GetStateSize());
    assert(r_.size() == GetOutputSize());
}

Eigen::VectorXd KalmanPlant::Measure() const {
    return LTISystem::GetOutput() + ctrl::random_from_variance(r_);
}

KalmanModel::KalmanModel(LTISystem sys, Eigen::VectorXd q, Eigen::VectorXd r)
    : LTISystem(sys), Q_(SquareDiagonal(q)), R_(SquareDiagonal(r)) {
    assert(q.size() == GetStateSize());
    assert(r.size() == GetOutputSize());

    Q_ = (q * q.transpose())
             .cwiseProduct(Eigen::MatrixXd::Identity(q.size(), q.size()));
    R_ = (r * r.transpose())
             .cwiseProduct(Eigen::MatrixXd::Identity(r.size(), r.size()));

    P_ = Eigen::MatrixXd(GetStateSize(), GetStateSize());
    P_.setZero();
}

void KalmanModel::Initialize(Eigen::VectorXd x0, Eigen::MatrixXd P0) {
    LTISystem::Initialize(x0);
    P_ = P0;
}

void KalmanModel::Predict(Eigen::VectorXd y) {
    auto K = P_ * C_.transpose() * (C_ * P_ * C_.transpose() + R_).inverse();
    X_ = X_ + K * (y - C_ * X_);  // update state estimate
    auto I = Eigen::MatrixXd::Identity(GetStateSize(), GetStateSize());
    P_ = (I - K * C_) * P_;  // update covariance
}

const Eigen::VectorXd KalmanModel::GetStateEstimate() const {
    return X_;
}

}  // namespace ctrl