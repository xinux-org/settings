{
  pkgs,
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
pkgs.stdenv.mkDerivation rec {
  pname = manifest.name;
  version = manifest.version;

  src = pkgs.lib.cleanSource ../..;
  cargoDeps = pkgs.rustPlatform.importCargoLock {
    lockFile = ../../Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    rustc
    # rustPlatform.cargoSetupHook # when you have cargoDeps
    cargo
    appstream
    appstream-glib
    desktop-file-utils
    gettext
    meson
    ninja
    pkg-config
    polkit
    wrapGAppsHook4
  ];

  buildInputs = with pkgs; [
    appstream
    appstream-glib
    desktop-file-utils
    gettext
    meson
    ninja
    pkg-config
    polkit
    wrapGAppsHook4
    rustPlatform.bindgenHook
  ];

}
