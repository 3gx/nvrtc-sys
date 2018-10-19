Raw wrappers to NVIDIA's nvrtc library.  See:

  https://docs.nvidia.com/cuda/nvrtc/index.html

In short, you can give this a string of CUDA code, and it will give back the
compiled object.

By default, this looks in /usr/local/cuda/ for headers and libraries.  If that
is not appropriate for your system, you can set the `CUDA_PATH` environment
variable.
