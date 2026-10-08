{
  pkgs,
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
pkgs.stdenv.mkDerivation {
  pname = manifest.name;
  version = manifest.version;

  src = pkgs.lib.cleanSource ../..;

  cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
    src = ../..;
    hash = "sha256-DfKt0hXEaB0L/910O0QSZUxW8a4WR2tg5Yxnodf21oQ=";
  };

  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    appstream
    appstream-glib
    desktop-file-utils
    gettext
    meson
    ninja
    pkg-config
    polkit
    libglycin
    glycin-loaders
    bubblewrap
    wrapGAppsHook4
    rustPlatform.cargoSetupHook
    rustPlatform.bindgenHook
    libinput
  ];

  buildInputs = with pkgs; [
    gtk4
    gnome-desktop
    libadwaita
    openssl
    vte-gtk4
    libgweather
    bubblewrap
    libglycin
    glycin-loaders
    imagemagick

    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
    gst_all_1.gst-plugins-ugly
    gst_all_1.gst-libav
  ];

  NIX_CFLAGS_COMPILE = "-DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -DMAGICKCORE_CHANNEL_MASK_DEPTH=32";
}
