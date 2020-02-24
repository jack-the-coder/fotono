# Fotono
"Fotono" is "photon" in Esperanto. 

## Goal
I'm building a toy path tracer to learn Rust. I'm told it's a good way to get a feel for a language and make something cool-looking in the process. Maybe at some point this will no longer be a toy, but for now that's the stage I'm looking to get to. To watch my progress, check out the Log section of this file.

## Resources
- [*The Rust Programming Language*](https://doc.rust-lang.org/book/)
- [*Physically-Based Rendering: From Theory to Implementation* (3rd ed.)](http://pbr-book.org/)
- [*Ray Tracing in One Weekend* series](https://raytracing.github.io/)

## Log
Newer entries should go on top. All of the times are a cumulative total since the beginning of the project. As you might expect, this is purely a side project, so I'm not putting a crazy amount of time into it.
### Feb 23 2020
| Item                                  | Total time spent learning (hours) |
|---------------------------------------|----------------------------------------------------|
| Learning Rust                         | 1                                                  |
| Reading PBRT and other graphics books | 1                                                  |
| Writing code for this project         | 0.25                                               |
#### Progress
We're currently producing images of the Julia set using the example from the `image` crate's documentation. The language itself seems very well-thought-out. I'm thinking of first building the path tracer in a single-threaded program and then parallelizing the actual ray-tracing part. This should be easy to make concurrent since there won't be any shared state or message passing. 

I'm not having any trouble grasping the vector math in the first few parts of *Ray Tracing in One Weekend*. 

Debug builds with `cargo` must include a lot of extra instrumentation! The 5000x5000 image takes a minute to generate with a debug build and about four seconds with a release build (i5-7400). Since I know that most of these programs take an absurdly long time to run, maybe I could use a big GCP machine to render on? 