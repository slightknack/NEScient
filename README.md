# NEScient
> *noun*  
> ne·​science | \ˈne-sh(ē-)ən(t)s, ˈnē-, -sē-ən(t)s\
> lack of knowledge or awareness: IGNORANCE

Nescient, stylized NEScient if you're having a bad day
and would like to be right on the internet for once,
is **a NES emulator with a simple implementation
and good performance**, written in Rust, inspired by
[fogleman/nes](https://www.github.com/fogleman/nes).

The goal of this project is of two parts:

1. To write a fast 'n functional NES emulator
   with minimal dependencies and clear cut behavior.
   No cruft or magic is allowed.
2. To write a small WASM backend for nescient,
   so it runs in the browser.

I've had some experience with WASM in the
past—I'm planning to use Vulkan as the graphics engine,
and still need to work out which audio engine
and input libraries to use.
so I'm primarily focused on that at the moment.

I'm not sure if vulkan is the best
choice—all I'm really doing is creating a window,
and writing a frame buffer.

## Roadmap
- Read through, port, and clean up fogleman/nes to Rust,
  to get a loose idea of NES emulator semantics.
- Looking through the implementation,
  I see some easy things that could be cleaned up,
  and if brainwill/nes is any indication,
  there are quite a few improvements that there can be made to
  the architecture of the emulator without changing behavior.
- Fix any large performance bottlenecks.
- Maybe get WASM working.
