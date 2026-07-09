# Rendy3D

A software renderer written from scratch in Rust.
It is being built for learning purposes so performance and features are likely not great

## Features

- Custom shaders (inspired by [euc](https://github.com/zesterer/euc))
- WASM support (see <https://tommypop2.github.io/Rendy3D/>)
- `no_std` support (can be used in embedded environments)
- Flexible mesh formats (can render anything that implements an `Iterator` that returns triangles)
- Triangle clipping

## TODO

- [x] Move application of the projection matrices into the shader
- [ ] Optimize triangle clipping (maybe do XY clipping in 2D on integer coordinates)
- [ ] Multithreading
- [ ] Shadows
- [ ] Accessing rate of change of interpolated values in the shader (allows for texture mipmapping)
- [ ] Decide on `Rasterizer` vs `Draw` trait - `Rasterizer` technically more flexible but requires creating a new struct. Could this just be solved by implementing draw on a newtype wrapper?

## Future Ideas

Experiment with shader composition (implementing a shader on top of an existing shader). This would allow for "pre-built" shaders that could do things like handle perspective rendering

Maybe a new crate for a more fully-featured engine?

Possibly seperate into renderer (literally just drawing triangles), rendering engine(cameras, shadows, mesh rendering), game engine(inputs, collisions, nicer transformation handling, etc...). The current `renderer` crate currently handles the first 2

## Game Engine Notes

See unity/godot for inspiration. The line between rendering engine and game engine can be a bit blurry. e.g should the camera/shadows be a feature in the renderer, or should the renderer render from one direction only & have the game engine itself handle cameras & rendering from different perspectives for shadows. Shadows are sort of a renderer job but cameras are an engine concern but the cameras are needed for shadows.

- Objects with transformations natively (no need to manipulate triangle arrays)
- Maybe collisions?
- Profiling tools (fps monitoring at a minimum)
- Input handling/event loop
- Renderer agnostic would be really cool (euc | some opengl/vulkan renderer)

Can test both with rubiks cube & mars lander project maybe? -> Maybe do rubiks renderer first and copy any useful abstractions into the engine

## Useful Resources

- <https://trenki2.github.io/blog/2017/06/06/developing-a-software-renderer-part1/>
- <https://elijahpotter.dev/articles/building_a_software_render_engine_from_scratch>
- <https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started>
- <https://lisyarus.github.io/blog/posts/implementing-a-tiny-cpu-rasterizer-part-1.html>
- <https://www.gabrielgambetta.com/computer-graphics-from-scratch>
- <https://terathon.com/blog/transforming-normals.html>
- <https://andrewkchan.dev/posts/perspective-interpolation.html>
- <https://songho.ca/opengl/gl_projectionmatrix.html>
