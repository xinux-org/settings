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
    gst_all_1.gst-vaapi
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
  '';

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
