# 🪵 Forge

cargo like tool for c and cpp

## 🍔 Installation

clone my repo and build using cargo

```bash
git clone https://github.com/aniket1275/forge.git
cargo build --release
```

## 🍁 Usage

```bash
forge new myproject
```

OR

```bash
forge new -l c++ myproject
```

- Create new project named myproject for c and cpp

```bash
forge init
```

OR

```bash
forge new -l c++ myproject
```

- Initalize project for c or cpp

```bash
forge run
```

- To compile and run the project

```bash
forge build
```

- To build the project

```bash
forge clean
```

- To delete the bin/ folder

## 🌸 Documentation

You can change the make file build, run and other Phony as you want.

Main files :

- forge.toml - for metadata about project
- Makefile - all make config
- src - for all c or cpp files
- include - for all c or cpp header files
