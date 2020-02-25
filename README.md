# Fotono
"Fotono" is "photon" in Esperanto. 

## Goal
I'm building a toy path tracer to learn Rust and computer graphics. I'm told it's a good way to get a feel for a language and make something cool-looking in the process. Maybe at some point this will no longer be a toy, but for now that's the stage I'm looking to get to. To watch my progress, check out the Log section of this file.

## Resources
- [*The Rust Programming Language*](https://doc.rust-lang.org/book/)
- [*Physically-Based Rendering: From Theory to Implementation* (3rd ed.)](http://pbr-book.org/)
- [*Ray Tracing in One Weekend* series](https://raytracing.github.io/)

## Log
Newer entries should go on top. All of the times are a cumulative total since the beginning of the project. As you might expect, this is purely a side project, so I'm not putting a crazy amount of time into it.
### Feb 24 2020
| Item                                  | Total time spent learning (hours) |
|---------------------------------------|----------------------------------------------------|
| Learning Rust                         | 2                                                  |
| Reading PBRT and other graphics books | 2                                                  |
| Writing code for this project         | 2.25                                               |
#### Progress
Whew! Today's been a whirlwind of compiler errors. Rust happens to have the most useful compiler errors I have ever experienced! They're simple, to-the-point, explain what you did wrong, and oftentimes offer links explaining some particular issue. 

The program now produces the sky image from the fourth section of *Ray Tracing in One Weekend*. I ended up using the `cgmath` crate for a nice implementation of `Vector3` instead of doing it myself. I might have learned more by building it by hand, but at the same time the `cgmath` implementation is pretty nice and offers some room to grow. I did make a `Ray` struct that uses `Vector3` under the hood. 

I don't forecast having quite this much time the rest of this week, so maybe I'll be at the "basic ray tracer" stage this weekend. Also, I'm thinking we're going to have to store the current image as a mutable 2D array of `u8`s so that each operation (painting the sky, tracing rays, tracing more rays, etc.) has something to write to other than the image itself. Maybe this is premature optimization, maybe not.

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