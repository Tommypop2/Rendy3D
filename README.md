# Rendy3D

A software renderer written from scratch in Rust.
It is being built for learning purposes so performance and features are likely not great

## Features

- Custom shaders (inspired by [euc](https://github.com/zesterer/euc))
- WASM support (see <https://tommypop2.github.io/Rendy3D/>)
- `no_std` support (can be used in embedded environments)
- Flexible mesh formats (can render anything that implements an `Iterator` that returns triangles)

## TODO

- [ ] Move application of the projection matrices into the shader
- [ ] Triangle clipping
- [ ] Shadows
- [ ] Accessing rate of change of interpolated values in the shader (allows for texture mipmapping)

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
- <https://songho.ca/opengl/gl_projectionmatrix.html>
