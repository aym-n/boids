## Boids Flocking Simulation in Rust with Nannou
![](https://github.com/aym-n/boids/blob/master/demo.gif?raw=true)
This project implements a flocking simulation based on Craig Reynolds' Boids algorithm using the Nannou graphics library in Rust.

### Overview

The Boids simulation models the flocking behavior of birds using simple rules for individual agents. These rules govern separation, alignment, and cohesion, leading to emergent flocking patterns without centralized control.
This project is inspired by Craig Reynolds' seminal work on Boids, which laid the groundwork for exploring complex emergent behavior in artificial life simulations.

### Implementation

The simulation consists of a group of boids (agents) with position, velocity, and orientation. Three core rules govern their behavior:

* **Separation:** Boids maintain a minimum distance from nearby neighbors to avoid overcrowding.
* **Alignment:** Boids adjust their direction to align with the average heading of neighboring boids.
* **Cohesion:** Boids steer towards the average position of nearby boids, promoting flock cohesion.

### Dependencies

This project requires the following dependencies:

* Rust (stable)
* Nannou graphics library

### Running the Simulation

1. Clone the repository:

   ```bash
   git clone https://github.com/aym-n/boids.git
   ```
2. Navigate to the project directory:

   ```bash
   cd boids
   ```
3. Install dependencies:

   ```bash
   cargo build --release
   ```
4. Run the simulation:

   ```bash
   cargo run
   ```
### References

* [Craig Reynolds' Boids](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/boids.html)

This project is a starting point for exploring the fascinating world of Boid simulations and emergent behavior in artificial life.
