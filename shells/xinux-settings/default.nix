{
  pkgs,
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
pkgs.mkShell {
  name = "${manifest.name}";

  # Compile time dependencies
  packages = with pkgs; [
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
    imagemagick

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
    libinput
    pkg-config
    gnome-desktop
    appstream
    appstream-glib
    wrapGAppsHook4
    mold
    desktop-file-utils
    gobject-introspection
    libglycin
    bubblewrap
    glycin-loaders
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
    gst_all_1.gst-plugins-ugly
    gst_all_1.gst-libav
    rustPlatform.bindgenHook
    udev
  ];

  buildInputs = with pkgs; [
    libinput
    gtk4
  ];

  shellHook = ''
    export XDG_DATA_DIRS="${pkgs.gtk4}/share:${pkgs.libadwaita}/share:${pkgs.gsettings-desktop-schemas}/share:${pkgs.glib}/share:$XDG_DATA_DIRS"
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=mold"
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=mold"
    export BINDGEN_EXTRA_CLANG_ARGS="$BINDGEN_EXTRA_CLANG_ARGS -DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -DMAGICKCORE_CHANNEL_MASK_DEPTH=32 -I${pkgs.imagemagick.dev}/include/ImageMagick-7";
  '';

  env = {
    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
    # Required env var for the `build.rs` of magick_rust to use tho correct ImageMagick CLANG flags.
    # This variable is used here: https://github.com/nlfiedler/magick-rust/blob/dfd8df0dd102348c23b33bfc946a9d70b5db25bf/build.rs#L127C9-L127C28
    # Not setting this variable will throw the error: "you should set MAGICKCORE_HDRI_ENABLE"
    "BINDGEN_EXTRA_CLANG_ARGS_${pkgs.stdenv.hostPlatform.rust.rustcTarget}" =
      "-DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -DMAGICKCORE_CHANNEL_MASK_DEPTH=32 -I${pkgs.imagemagick.dev}/include/ImageMagick-7";
  };

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
