# The Ray Tracer Challenge

```
mkdir _ninja && cd _ninja
conan install .. --build=missing
cmake .. -GNinja
ninja
```
## Ideas
- add sanitisers
- move methods onto classes
- have specific classes for different matrices, e.g. a 2x2 class, 3x3 class, etc.
- all matrices are square, so replace width and height with size
- fluent api for chaining transformations
