raytracing-weekend
======

My Rust port of the book [Ray Tracing in One Weekend](https://github.com/petershirley/raytracinginoneweekend)

The output of the program is a PPM formatted image echo'ed to stdout via `println!`. To get a file, just pipe stdout to file. Gimp can open PPM result.

Running
=======
`cargo run --release > raytraced.ppm`