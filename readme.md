# Vertex

A modern, statically-typed programming language designed for clarity, performance, and compile-time safety.

## ✨ Key Features

- **Statically Typed**: Catch all type errors at compile time, ensuring code reliability
- **Fast Compilation**: Compile your programs in seconds, not minutes
- **Bytecode Compilation**: Programs are compiled to efficient bytecode for predictable performance
- **Type Inference**: Write less boilerplate while maintaining type safety
- **Simple Syntax**: Clean, readable syntax inspired by modern languages

## 🚀 Quick Start

### Installation

**Prerequisites:**
- Rust toolchain
- Python 3+
- Git

**Build from Source:**
```bash
git clone https://github.com/DomioKing653/Vertex.git
cd Vertex
python install.py
```

The compiled binary will be available in `./target/release/`

For detailed installation instructions, see [INSTALLATION.md](INSTALLATION.md).

### Your First Program

```vertex
fn main() {
    print("Hello, Vertex!")
}
```

Learn more in our [Getting Started Guide](docs/GETTING_STARTED.md).

## 📖 Documentation

Complete language documentation, tutorials, and examples are available in the [docs](docs/) directory.

## 🔧 Project Structure

```
vertex/
├── src/              # Compiler source code
├── std/              # Standard library
├── testingCode/      # Test cases
├── docs/             # Language documentation
└── Cargo.toml        # Project manifest
```

## 💻 Platform Support

| Platform | Status |
|----------|--------|
| Linux    | ✅ Fully Supported |
| macOS    | 🔧 Build from source |
| Windows  | 🔧 Build from source |

Pre-built binaries are currently available for Linux. For other platforms, please build from source.

## 🛠️ Building from Source

```bash
# Clone the repository
git clone https://github.com/DomioKing653/Vertex.git
cd Vertex

# Build the compiler
cargo build --release

# Run tests
./testing_script.sh
```

## 📋 Current Status

- **Version**: 0.0.32-alpha
- **Status**: Early development
- **License**: [MIT](license.md)

This is an active project in early alpha stages. Features and syntax may change.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

Vertex is licensed under the [MIT License](license.md).

---

**Questions?** Open an [issue](https://github.com/DomioKing653/Vertex/issues) or check the [documentation](docs/).