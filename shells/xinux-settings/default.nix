{
  pkgs ?
    let
      lock = (builtins.fromJSON (builtins.readFile ../../flake.lock)).nodes.nixpkgs.locked;
      nixpkgs = fetchTarball {
        url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
        sha256 = lock.narHash;
      };
    in
    import nixpkgs { overlays = [ ]; },
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
pkgs.mkShell {
  name = "${manifest.name}";

  # Compile time dependencies
  nativeBuildInputs = with pkgs; [
    # Hail the Nix
    nixd
    statix
    deadnix
    nixfmt

    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch
    openssl
    just
    just-lsp
    # Gnome related
    gtk4
    meson
    mesonlsp
    ninja
    pango
    polkit
    gettext
    vte-gtk4
    libgweather
    pkg-config
    gdk-pixbuf
    libadwaita
    pkg-config
    gnome-desktop
    appstream
    appstream-glib
    wrapGAppsHook4
    desktop-file-utils
    gobject-introspection
    rustPlatform.bindgenHook
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
