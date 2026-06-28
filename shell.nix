{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "swa-dev";

  buildInputs = with pkgs; [
    # Rust toolchain
    rustup
    rust-analyzer

    # LLVM — pinned to 22.x for reproducible builds
    llvm_22
    llvmPackages_22.libllvm
    llvmPackages_22.libclang

    # Linker and build tools
    lld_22
    cmake
    ninja
    pkg-config

    # Native deps for tests
    gcc
    binutils
  ];

  shellHook = ''
    export LLVM_CONFIG="${pkgs.llvm_22}/bin/llvm-config"
    export LLVM_SYS_220_PREFIX="${pkgs.llvmPackages_22.libllvm.dev}"
    export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
    echo "swa dev shell — LLVM $(llvm-config --version), Rust $(rustc --version)"
  '';
}
