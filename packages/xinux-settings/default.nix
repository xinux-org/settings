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
    hash = "sha256-WB8AwWFquuyHfNQilYpuEi2umtpSWa6ASnfgv4zXYv8=";
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
