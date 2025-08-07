# Rendy3D

A software renderer written from scratch in Rust.
It is being built for learning purposes so performance and features are likely not great

## TODO

- [ ] `RenderTarget` trait, so other buffers can be rendered to (e.g stencil buffer)
- [ ] Support panning and zooming around the viewport
- [ ] Pixel shaders
- [ ] Apply vertex shaders efficiently to every vertex
  - Don't know how to do this yet without wasting work (applying multiple times to the same vertex if it appears in multiple triangles), or allocating memory
- [ ] Compile as WASM and allow running in the browser

## Useful Resources

- <https://trenki2.github.io/blog/2017/06/06/developing-a-software-renderer-part1/>
- <https://elijahpotter.dev/articles/building_a_software_render_engine_from_scratch>
- <https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started>
- <https://lisyarus.github.io/blog/posts/implementing-a-tiny-cpu-rasterizer-part-1.html>
