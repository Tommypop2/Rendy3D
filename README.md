# Rendy3D

A software renderer written from scratch in Rust.
It is being built for learning purposes so performance and features are likely not great

## TODO

- [x] `RenderTarget` trait, so other buffers can be rendered to (e.g stencil buffer)
- [x] Support panning and zooming around the viewport
- [x] Pixel shaders
- [x] Apply vertex shaders efficiently(\* still applied multiple times) to every vertex
  - Don't know how to do this yet without wasting work (applying multiple times to the same vertex if it appears in multiple triangles), or allocating memory
- [x] Compile as WASM and allow running in the browser
- [x] Support Texturing

## Future Ideas

Experiment with shader composition (implementing a shader on top of an existing shader). This would allow for "pre-built" shaders that could do things like handle perspective rendering

Maybe a new crate for a more fully-featured engine?

## Useful Resources

- <https://trenki2.github.io/blog/2017/06/06/developing-a-software-renderer-part1/>
- <https://elijahpotter.dev/articles/building_a_software_render_engine_from_scratch>
- <https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started>
- <https://lisyarus.github.io/blog/posts/implementing-a-tiny-cpu-rasterizer-part-1.html>
- <https://www.gabrielgambetta.com/computer-graphics-from-scratch>
- <https://terathon.com/blog/transforming-normals.html>
- <https://andrewkchan.dev/posts/perspective-interpolation.html>
