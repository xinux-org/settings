global_conf = configuration_data()
global_conf.set_quoted('APP_ID', application_id)
global_conf.set_quoted('VERSION', version + version_suffix)
global_conf.set_quoted('PKGDATADIR', pkgdatadir)
global_conf.set_quoted('PROFILE', profile)
global_conf.set_quoted('VERSION', version + version_suffix)
global_conf.set_quoted('GETTEXT_PACKAGE', gettext_package)
global_conf.set_quoted('LOCALEDIR', localedir)
global_conf.set_quoted('LIBEXECDIR', libexecdir)
global_conf.set_quoted('RESOURCES_FILE', pkgdatadir / 'resources.gresource')

config_rs = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: global_conf
)

run_command(
  'cp',
  config_rs,
  meson.current_source_dir(),
  check: true
)

cargo_build_options = [ 
  '--manifest-path', meson.project_source_root() / 'Cargo.toml',
  cargo_release_option
]
custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  depends: resources,
  env: cargo_env,
  command: [
    cargo, 'build',
    cargo_build_options,
    '&&',
    'cp', cargo_target / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
