#pragma once

#include "Eigen/Dense"

namespace ctrl {

/// @brief Generate a vector of equally spaced points.
///
/// @param start Low point
/// @param stop High point
/// @param num_points Number of points
/// @param endpoint True to include stop, False to not.
/// @return Eigen::VectorXd
Eigen::VectorXd linspace(double start, double stop, int num_points,
                         bool endpoint);

}  // namespace ctrl
