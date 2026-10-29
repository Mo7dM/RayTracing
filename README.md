# Ray tracer

A Rust ray tracer that writes ASCII PPM images. It supports spheres, flat planes, axis-aligned cubes and capped vertical cylinders, with a point light, diffuse shading and hard shadows. No external dependencies are needed.

## Rendering

```sh
cargo run --release -- sphere > sphere.ppm
cargo run --release -- cube > cube.ppm
cargo run --release -- all > all.ppm
cargo run --release -- alternate > alternate.ppm
```

The four scenes render at 800×600 by default. `cube` has lower light intensity than `sphere`; `all` and `alternate` use the same objects and light with different camera positions. For a faster preview specify width and height, for example `cargo run --release -- all 400 300 > preview.ppm`. Progress and errors go to standard error.

## Editing scenes

Scene setup lives in `src/main.rs`. Positions are `(x, y, z)`. Add objects to `world` as follows; the final vector gives red, green and blue values from 0 to 1:

```rust
world.add(Box::new(ColoredObject::new(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)), Color::new(1.0, 0.0, 0.0))));
world.add(Box::new(ColoredObject::new(Box::new(Cube::new(Point3::new(-2.0, -0.15, -1.0), 0.7)), Color::new(0.0, 0.0, 1.0))));
world.add(Box::new(ColoredObject::new(Box::new(Cylinder::new(Point3::new(1.5, -0.15, -2.0), 0.3, 0.7)), Color::new(0.0, 1.0, 0.0))));
world.add(Box::new(ColoredObject::new(Box::new(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0))), Color::new(0.6, 0.6, 0.6))));
```

The cube size is its side length; the cylinder arguments are center, radius and height. The plane arguments are a point and a nonzero normal. Modify the `Light::new` intensity to change brightness. Change `camera_position` to move the camera and `target` to aim it. Avoid a camera position equal to the target or directly above/below it, because the camera's up vector would be undefined. Change `viewport_height` to zoom.

For each pixel, the renderer finds the nearest hit, uses its surface normal to calculate diffuse light, and casts a shadow ray towards the light. A small ambient term keeps unlit surfaces visible.

## References

- [01 Edu assignment](https://github.com/01-edu/public/tree/master/subjects/rt)
- [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app/)
