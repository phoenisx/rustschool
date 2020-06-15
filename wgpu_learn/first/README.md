We will be building a simple triangle
to understand the basics of RenderPass, Pipelines and shaders

## How to convert GLSL files to SPV

As you can see we have two shader files in `/resources` folder,
one is `fragment` shader stage and other is `vertext` shader stage.

For eg., compiling fragment shaders to respective `.spv` output,
we can use the following commands.

```sh
glslc \
  -fshader-stage=frag \
  -std=450 \
  -O -g \
  -o ./resources/triangle.frag.spv \
  ./resources/triangle.frag
```
