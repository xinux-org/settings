use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub struct LoadOutput {
    pub config: nix_data::config::configfile::NixDataConfig,
    pub flakepath: PathBuf,
}

pub fn load() -> Result<LoadOutput> {
    let config = nix_data::config::configfile::getconfig().expect("Failed to load config");
    let flakepath = config
        .flake
        .as_ref()
        .map(PathBuf::from)
        .expect("Failed to get flake path");
    Ok(LoadOutput { config, flakepath })
}

pub fn loadanyconfig(target_config_file: &PathBuf) -> Result<String> {
    fs::read_to_string(target_config_file).context("relative_path")
}

// /**
//  * Load current module configuration in `modules.nix`, located next to the `default.nix` file.
//  */
// pub fn loadmoduleconfig(config: &NixDataConfig) -> Result<String> {
//     let modulesnix = Path::new(&config.systemconfig.clone().context("systemconfig")?)
//         .parent()
//         .context("systemconfig parent")?
//         .join("modules.nix");
//     fs::read_to_string(modulesnix).context("modules.nix")
// }

// pub fn getmodulepath(config: &NixDataConfig) -> Result<PathBuf> {
//     let modulesnix = Path::new(&config.systemconfig.clone().context("systemconfig")?)
//         .parent()
//         .context("systemconfig parent")?
//         .join("modules.nix");
//     Ok(modulesnix)
// }

// pub fn getcurrentoptions(
//     config: &NixDataConfig,
//     modules: &[Module],
// ) -> Result<HashMap<String, ModuleOption>> {
//     let modulesnix = Path::new(&config.systemconfig.clone().context("systemconfig")?)
//         .parent()
//         .context("systemconfig parent")?
//         .join("modules.nix");
//     let moduletext = fs::read_to_string(modulesnix).context("modules.nix")?;

//     let options = modules
//         .iter()
//         .map(|x| x.config.options.clone())
//         .collect::<Vec<_>>()
//         .concat();

//     let mut output = HashMap::new();
//     for option in options {
//         let attribute = option.id;
//         let string_value = nix_editor::read::readvalue(&moduletext, &attribute);

//         if let Ok(string_value) = string_value {
//             match option.op_type {
//                 OptionType::Switch { .. } => {
//                     let value = match string_value.as_str() {
//                         "true" => true,
//                         "false" => false,
//                         _ => continue,
//                     };
//                     output.insert(attribute, ModuleOption::Switch { value });
//                 }
//                 OptionType::Text { .. } => {
//                     let value = string_value
//                         .strip_prefix('"')
//                         .and_then(|x| x.strip_suffix('"'));
//                     if let Some(value) = value {
//                         output.insert(
//                             attribute,
//                             ModuleOption::Text {
//                                 value: value.to_string(),
//                             },
//                         );
//                     }
//                 }
//                 OptionType::Enum { options, .. } => {
//                     output.insert(
//                         attribute,
//                         ModuleOption::Enum {
//                             value: string_value.to_string(),
//                             pretty: if let Some(pretty) = options.get(&string_value) {
//                                 pretty.to_string()
//                             } else {
//                                 string_value.to_string()
//                             },
//                         },
//                     );
//                 }
//                 OptionType::NumberList { .. } => {
//                     if let Ok(arr) = nix_editor::read::getarrvals(&moduletext, &attribute) {
//                         let numbers = arr
//                             .iter()
//                             .filter_map(|x| x.parse::<u32>().ok())
//                             .collect::<Vec<_>>();
//                         output.insert(attribute, ModuleOption::NumberList { value: numbers });
//                     }
//                 }
//             }
//         }
//     }
//     Ok(output)
// }
