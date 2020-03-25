# Add Background

## Creating our Initial States

Following are the important states, that let's our Vulkan Application to actually allow
to do something, later on.

```rs
pub struct BackendState<B: Backend> {
    instance: Option<B::Instance>,
    surface: ManuallyDrop<B::Surface>,
    adapter: AdapterState<B>,
    window: winit::window::Window,
}

struct AdapterState<B: Backend> {
    adapter: Option<Adapter<B>>,
    memory_types: Vec<MemoryType>,
    limits: Limits,
}
```

* Vulkan Instance: is created for specific GPU Backend, like Vulkan. `back` denotes to a specific Backend.
```rs
let instance = back::Instance::create("Awesome Backend", 0).expect("Initializing Failed");
```
