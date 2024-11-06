# Examples

## LTI

Simulates a car accelerating and coasting with drag.

```bash
make examples/lti.cpp
```

**Usage:** `./lti <drag> <timestep>`. Pipe the output to a file called `lti.out` then run `gnuplot`.

### Example

```bash
./build/examples/lti.exe 0.6 0.1 > lti.out
gnuplot examples/lti.plot
```

![LTI Example](img/lti.png)
