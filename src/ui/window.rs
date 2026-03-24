use anyhow::Context;
use nix_data::config::configfile::NixDataConfig;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw::{self, prelude::*},
    gtk::{gio, glib},
    *,
};

use crate::ui::search::SearchModal;
use crate::ui::{
    about::AboutDialog, accessibility::AccessibilityModel, accounts::AccountsModel,
    bluetooth::BluetoothModel, display::DisplayModel, mouse::MouseAndTouchpad,
    multitasking::MultitaskingModel, network::NetworkModel, notifications::NotificationsModel,
    power::PowerModel, privacyandsecurity::PrivacyAndSecurityModel, sharing::SharingModel,
    sound::SoundModel, system::SystemPageModel, wellbeing::WellbeingModel, wifi::WifiModel,
    appearance::AppearanceModel,
};
use crate::ui::{apps::AppModal, rebuild::rebuild_dialog::RebuildInput};
use crate::ui::appearance;
use crate::utils::modules::load::LoadOutput;
use crate::{
    config::{APP_ID, PROFILE},
    ui::rebuild::rebuild_dialog::{RebuildInit, RebuildModel},
};
use crate::ui::{apps::AppModal, rebuild::rebuild_dialog::RebuildInput};
use crate::utils::modules::load::LoadOutput;
use crate::{
    config::{APP_ID, PROFILE},
    ui::rebuild::rebuild_dialog::{RebuildInit, RebuildModel},
};

use std::{convert::identity, fs, path::Path};

pub struct App {
    _wifi: Controller<WifiModel>,
    _network: Controller<NetworkModel>,
    _bluetooth: Controller<BluetoothModel>,
    _display: Controller<DisplayModel>,
    _appearance: Controller<AppearanceModel>,
    _sound: Controller<SoundModel>,
    _power: Controller<PowerModel>,
    _multitasking: Controller<MultitaskingModel>,
    _apps: Controller<AppModal>,
    _notifications: Controller<NotificationsModel>,
    _search: Controller<SearchModal>,
    _accounts: Controller<AccountsModel>,
    _sharing: Controller<SharingModel>,
    _wellbeing: Controller<WellbeingModel>,
    _mouse: Controller<MouseAndTouchpad>,
    _accessibility: Controller<AccessibilityModel>,
    _privacyandsecurity: Controller<PrivacyAndSecurityModel>,
    _system: Controller<SystemPageModel>,

    config: NixDataConfig,
    rebuild_dialog: Controller<RebuildModel>,
    // error_dialog: Controller<ErrorDialogModel>,
    // moduleconfig: String,

    // current_config: HashMap<String, ModuleOption>,
    // modified_config: HashMap<String, ModuleOption>,
}

pub struct AppInit {
    pub load: LoadOutput,
}

#[derive(Debug)]
pub enum AppMsg {
    Rebuild(String, String, String), // single line nix path, argument and value
    Reload,
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = AppInit;
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About" => AboutAction,
            }
        }
    }and

    view! {
    #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            // #[wrap(Some)]
            // set_help_overlay: shortcuts = &gtk::Builder::from_resource(
            //         "/uz/xinux/Settings/gtk/help-overlay.ui"
            //     )
            //     .object::<gtk::ShortcutsWindow>("help_overlay")
            //     .unwrap() -> gtk::ShortcutsWindow {and
            //         set_transient_for: Some(&main_window),
            //         set_application: Some(&main_application()),
            // },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            #[name(split_view)]
            adw::NavigationSplitView {
                // set_min_sidebar_width: 180.0,

                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "Settings",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            pack_end = &gtk::MenuButton {
                                set_icon_name: "open-menu-symbolic",
                                set_menu_model: Some(&primary_menu),
                            }
                        },

                        #[wrap(Some)]
                        set_content = &gtk::StackSidebar {
                            set_stack: &stack,
                        },
                    },
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    // set_title: "Content",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        // add_top_bar = &adw::HeaderBar {},
                        set_content: Some(&stack),
                    }
                },
                },
and
                add_breakpoint = bp_with_setters(
                    adw::Breakpoint::new(
                        adw::BreakpointCondition::new_length(
                            adw::BreakpointConditionLengthType::MaxWidth,
                            400.0,
                            adw::LengthUnit::Sp,
                        )
                    ),
                    &[
                    (&split_view, "collapsed", true),
                    // (&model.welcome_page.widget(), "reveal", Some(&true.into()))
                    ]
                ),
        },
        stack = &gtk::Stack {
            add_titled: (wifi.widget(), Some("wifi"), "Wi-Fi"),
            add_titled: (network.widget(), Some("network"), "Network"),
            add_titled: (bluetooth.widget(), Some("bluetooth"), "Bluetooth"),
            add_titled: (display.widget(), Some("display"), "Display"),
            add_titled: (appearance.widget(), Some("appearance"), "Appearance"),
            add_titled: (sound.widget(), Some("sound"), "Sound"),
            add_titled: (power.widget(), Some("power"), "Power"),
            // add_titled: (multitasking.widget(), Some("multitasking"), "Multitasking"),
            add_titled: (apps.widget(), Some("apps"), "Apps"),
            add_titled: (notifications.widget(), Some("notifications"), "Notifications"),
            // add_titled: (search.widget(), Some("search"), "Search"),
            // add_titled: (accounts.widget(), Some("accounts"), "Online Accounts"),
            // add_titled: (sharing.widget(), Some("sharing"), "Sharing"),
            // add_titled: (wellbeing.widget(), Some("wellbeing"), "Wellbeing"),
            add_titled: (mouse.widget(), Some("mouse"), "Mouse and Touchpad"),
            // add_titled: (accessibility.widget(), Some("accessibility"), "Acccesibility"),
            // add_titled: (privacyandsecurity.widget(), Some("privacyandsecurity"), "Privacy and Security"),
            add_titled: (system.widget(), Some("system"), "System"),
            set_vhomogeneous: false,
            set_hhomogeneous: false,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let LoadOutput { config, flakepath } = init.load;

        let wifi = WifiModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let network = NetworkModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let bluetooth = BluetoothModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let display = DisplayModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let appearance = AppearanceModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let sound = SoundModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let power = PowerModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let multitasking = MultitaskingModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let apps = AppModal::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let notifications = NotificationsModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let search = SearchModal::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let accounts = AccountsModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let sharing = SharingModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let wellbeing = WellbeingModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let mouse = MouseAndTouchpad::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let accessibility = AccessibilityModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let privacyandsecurity = PrivacyAndSecurityModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let system = SystemPageModel::builder()
            .launch(())and
            .forward(sender.input_sender(), identity);

        let rebuild_dialog = RebuildModel::builder()
            .transient_for(&root)
            .launch(RebuildInit {
                flakepath,
                // modulepath,
                generations: config.generations,
            })
            .forward(sender.input_sender(), identity);

        let rebuild_dialog = RebuildModel::builder()
            .transient_for(&root)
            .launch(RebuildInit {
                flakepath,
                // modulepath,
                generations: config.generations,
            })
            .forward(sender.input_sender(), identity);

        let widgets = view_output!();

        let model = App {
            _wifi: wifi,
            _network: network,
            _bluetooth: bluetooth,
            _display: display,
            _appearance: appearance,
            _sound: sound,
            _power: power,
            _multitasking: multitasking,
            _apps: apps,
            _notifications: notifications,
            _search: search,
            _accounts: accounts,
            _sharing: sharing,
            _wellbeing: wellbeing,
            _mouse: mouse,
            _accessibility: accessibility,
            _privacyandsecurity: privacyandsecurity,
            _system: system,

            config,
            rebuild_dialog,
            // modified_config: HashMap::new(),
        };

        widgets.stack.connect_visible_child_notify({
            let split_view = widgets.split_view.clone();
            move |_| {
                split_view.set_show_content(true);
            }
        });

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        // let shortcuts_action = {
        //     let shortcuts = widgets.shortcuts.clone();
        //     RelmAction::<ShortcutsAction>::new_stateless(move |_| {
        //         shortcuts.present();
        //     })
        // };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        // actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Rebuild(relative_config_path, argument, value) => {
                // path to be written arg and val usually inside ./modules/nixos/. not configuration.nix
                let full_config_path = Path::new(&self.config.flake.clone().unwrap())
                    .parent()
                    .context("systemconfig parent")
                    .unwrap()
                    .join(relative_config_path);

                // String type readed file. e.x: {}, "{...}:\n{\n  i18n.defaultLocale..
                let full_config_string = fs::read_to_string(&full_config_path)
                    .context("String type readed file")
                    .unwrap();

                // new changed file to be written in s-helper and saved/overwritten
                let output = nixpkgs_fmt::reformat_string(
                    &nix_editor::write::write(
                        &full_config_string,
                        &argument,
                        &format!("\"{}\"", value),
                    )
                    .unwrap(),
                );

                self.rebuild_dialog.emit(RebuildInput::Rebuild(
                    // self.modified_config.clone(),
                    output.to_owned(),
                    full_config_path.into_os_string().into_string().unwrap(),
                ))
            }
            AppMsg::Reload => {}

            AppMsg::Quit => main_application().quit(),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

fn bp_with_setters(
    bp: adw::Breakpoint,
    additions: &[(&impl IsA<glib::Object>, &str, impl ToValue)],
) -> adw::Breakpoint {
    bp.add_setters(additions);
    bp
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}

