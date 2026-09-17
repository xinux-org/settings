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
    src = pkgs.lib.cleanSource ../..;
    hash = "sha256-lnXPU4uLnByv8H9SaKfCXuxPcM0gACC2tGGtqDceEzY=";
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

    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
    gst_all_1.gst-plugins-ugly
    gst_all_1.gst-libav
  ];

}
