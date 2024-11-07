#pragma once

#include <random>

#include "Eigen/Dense"
#include "state_space.hpp"
#include "util.hpp"

namespace ctrl {

class KalmanPlant : public LTISystem {
public:
    KalmanPlant(LTISystem sys, Eigen::VectorXd q, Eigen::VectorXd r);

    template <typename T>
    void Update(T u);
    Eigen::VectorXd Measure() const;

private:
    Eigen::VectorXd q_;  // model noise
    Eigen::VectorXd r_;  // measurement noise
};

class KalmanModel : protected LTISystem {
public:
    KalmanModel(LTISystem sys, Eigen::VectorXd q, Eigen::VectorXd r);
    void Initialize(Eigen::VectorXd x0, Eigen::MatrixXd P0);

    template <typename T>
    void Update(T u);

    void Predict(Eigen::VectorXd measurement);

    const Eigen::VectorXd GetStateEstimate() const;

private:
    Eigen::MatrixXd Q_;  // model noise
    Eigen::MatrixXd R_;  // measurement noise
    Eigen::MatrixXd P_;  // covariance
};

/***************************************************************
    Template method implementations
***************************************************************/

template <typename T>
void KalmanPlant::Update(T u) {
    LTISystem::Update(u);
    X_ += random_from_variance(q_);
}

template <typename T>
void KalmanModel::Update(T u) {
    LTISystem::Update(u);
    P_ = A_ * P_ * A_.transpose() + Q_;
}

}  // namespace ctrl