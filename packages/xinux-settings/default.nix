{
  pkgs,
  inputs,
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;

  craneLib = inputs.crane.mkLib pkgs;

  commonBuildInputs = with pkgs; [
    gtk4
    gnome-desktop
    libadwaita
    openssl
    vte-gtk4
    libgweather
  ];

  commonNativeBuildInputs = with pkgs; [
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

  cargoArtifacts = craneLib.buildDepsOnly {
    src = craneLib.cleanCargoSource ../../.;
    strictDeps = true;

    nativeBuildInputs = commonNativeBuildInputs;
    buildInputs = commonBuildInputs;
  };
in
craneLib.buildPackage {
  pname = manifest.name;
  version = manifest.version;
  strictDeps = true;

  src = pkgs.lib.cleanSource ../../.;

  cargoDeps = pkgs.rustPlatform.importCargoLock {
    lockFile = ../../Cargo.lock;
  };

  inherit cargoArtifacts;

  nativeBuildInputs = commonNativeBuildInputs;
  buildInputs = commonBuildInputs;

  preConfigure = ''
    mesonFlagsArray+=("-Dcargo_home=$CARGO_HOME")
  '';

  configurePhase = ''
    mesonConfigurePhase
    runHook postConfigure
  '';

  buildPhase = ''
    runHook preBuild
    ninjaBuildPhase
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mesonInstallPhase
    runHook postInstall
  '';

  doNotPostBuildInstallCargoBinaries = true;
  checkPhase = false;
}
