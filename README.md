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
forge new -l c myproject
```

- Create new project named myproject

```bash
forge init -l c
```

- Initalize in folder

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

Main files :

- forge.toml - for metadata about project
- Makefile - all make config
- src - for all c or cpp files
- include - for all c or cpp header files
