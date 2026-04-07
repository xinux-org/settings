project(
  'settings',
  'rust',
  version: '0.1.0',
  meson_version: '>= 1.4',
  license: 'GPL_2_0_ONLY',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'uz.xinux.Settings'

dependency('glib-2.0', version: '>= 2.76')
dependency('gio-2.0', version: '>= 2.76')
dependency('gtk4', version: '>= 4.15.3')
dependency('libadwaita-1', version :'>=1.8')

cargo = find_program('cargo', required: true)
gresource = find_program('glib-compile-resources', required: true)
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstreamcli = find_program('appstreamcli', required: false)

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')
libexecdir = prefix / get_option('libexecdir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()

if get_option('profile') == 'development'
  message('Building in debug mode')
  profile = 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD').stdout().strip()

  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format (vcs_tag)
  endif
  application_id = '@0@.@1@'.format(base_id, profile)

  rust_target = 'debug'
  cargo_release_option = []
else
  message('Building in release mode')
  profile = ''
  version_suffix = ''
  application_id = base_id

  rust_target = 'release'
  cargo_release_option = [ '--release' ]
endif

cargo_home = get_option('cargo_home')

if cargo_home == ''
  cargo_home = meson.project_build_root() / 'cargo-home'
endif

cargo_target = meson.project_build_root() / 'cargo-target'
cargo_env = environment()
cargo_env.set('CARGO_HOME', cargo_home)
cargo_env.set('CARGO_TARGET_DIR', cargo_target)
meson.add_devenv(cargo_env)

find_program('cargo-vendor-filterer', required: false)
meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  cargo_home,
  cargo_target,
)

if get_option('profile') == 'development'
  # Setup pre-commit hook for ensuring coding style is always consistent
  message('Setting up git pre-commit hook..')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit', check: false)
endif

subdir('data')
subdir('po')
subdir('src')
subdir('s-helper/src')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)

# see more: https://github.com/alatiera/gnome-podcasts/blob/main/meson.build
# test_script = find_program('scripts/test.sh', required: true)
# test(
#   'cargo-test',
#   test_script,
#   args: [
#     cargo_release_option,
#     '--all-targets',
#     '--workspace',
#   ],
#   env: cargo_env,
#   depends: podcasts_resources,
#   workdir: meson.project_source_root(),
#   timeout: 3000
# )