## Vulkan Instance and Surface
![Vulkan Flow Image](https://user-images.githubusercontent.com/11786283/77244041-e4bf5680-6c36-11ea-96a1-2be7bd71ccbd.png)

This Tutorial is inspired from [LunarG Vulkan Tutorial](https://vulkan.lunarg.com/doc/sdk/1.2.131.2/linux/tutorial/html/index.html),
which is written in `C++`. I am trying to learn `Rust` and converting that tutorial into `Rust`,
using `gfx-hal` library, which is a wrapper over Vulkan Specs.

* Application: is what we will build using `gfx-hal`
* Loader: here refers to `gfx-hal` and `gfx-backend-vulkan` libraries. An isntance of
  `gfx-backend-vulkan` inititalizes a Loader.
* Layers: is something advanced, and am not sure when or if I will talk about it at-all.

## Backend
<img alt="GFX Hal Backends" src="https://user-images.githubusercontent.com/11786283/77244047-03255200-6c37-11ea-885d-2d8b981bb8a8.png" />

Backends are specific to what GPU u have and what specs it supports.

Vulkan Backend is cross-compatible and has support in Linux/Widnows, on AMD, Intel, NVidia etc.
> Apple stays out, and I hate this thing about it, it doesn't support Vulkan, and has speicifc
> backend in Metal. Though `gfx-hal` has `metal` backend as well.

To work with `gfx-hal` we need to create an instance of a specific Backend which can be controlled using Rust Feature Configurations.

```rs
#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
```

## Instance

Vulkan instance is the starting point to work with Vulkan. A Vulkan instance actually takes the name
of Application, and provides us with various APIs, specially to `enumerate_devices`.

```rs
// create(application_name, application_version);
let instance = back::Instance::create("Backend Instance", 1)
  .expect("Failed to create an instance!");
```

## Surface

Vulkan requires a canvas or `surface` to draw things into and a `surface` can only exist inside
a OS App Window. Usually we will be using a 3rd-party module to create OS specific Window instances,
like `winit`, and thw window instance to create a `surface`.

```rs
// `wb` is `winit` window_builder.
let window = wb.build(event_loop).unwrap();

// Create the Surface:
let surface = unsafe {
  instance
    .create_surface(&window)
    .expect("Failed to create a surface!")
};
```
