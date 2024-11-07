# Control

Implementing concepts from MECHTRON 4AX3 - Predictive and Intelligent Control.

## Examples

### LTI State Space

Simulates a car accelerating and coasting with drag. Accepts two arguments: `<drag> <timestep>`.

```bash
make examples/lti.cpp
./build/examples/lti.exe 0.6 0.1 > series.out
gnuplot examples/series.plot
```

![LTI Example](img/lti.png)

### Kalman Filter

Simulate the same car accelerating then coasting, but now a Kalman filter tries to estimate the position from noisy measurements.

```bash
make examples/kalman.cpp
./build/examples/kalman.exe .1 1 > series.out
gnuplot examples/series.plot
```

![Kalman Example](img/kalman.png)

## Roadmap

- ~~Linear Least Squares via Normal Equation~~
- ~~Gradient Descent solver~~
- ~~State space classes & solving methods~~
- ~~Kalman Filter~~
- Luenberger Observer
- Extended Kalman filter
  - Is this code any different than the standard KF?
- Control
  - Converging to optimal policy
  - Learning
  - Exploration
