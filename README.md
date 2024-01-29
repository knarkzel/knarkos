# knarkos

What if Emacs was the entire operating system, written in a modern
programming language? Basically the entire operating system is a lisp-runtime
that runs `.lisp` files and compiles them to bytecode. Everything else can built
on top of this. Each binary would be tiny with no overhead of including a runtime,
because the operating system itself is the runtime.

```bash
git clone https://github.com/knarkzel/knarkos
cd knarkos 
nix develop # or nix-shell
just build
```

## Goals

- Not based on Unix
- Simplicity from Plan9
- Reproducable OS like NixOS?
- Written in Rust for safety & performance
- Emacs as a complete OS, everything powered by Lisp
- Network of Lisp-powered machines, bee-hive concept

## Features

- [ ] 2D graphics
- [ ] Networking
- [ ] Lisp bytecode VM
- [ ] Hot reloading everywhere

## Resources

- [Compiling a Lisp](https://bernsteinbear.com/blog/compiling-a-lisp-1/)
- [Bytecode interpreters](https://bernsteinbear.com/blog/bytecode-interpreters/)
