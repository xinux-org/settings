use anyhow::Context;
use gettextrs::gettext;
use nix_data_xinux::config::configfile::NixDataConfig;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw::{self, prelude::*},
    gtk::{gio, glib},
    prelude::{AsyncComponent, AsyncComponentController, AsyncController},
    *,
};

use crate::ui::{
    about::AboutDialog, accessibility::AccessibilityModel, accounts::AccountsModel,
    appearance::appearance::AppearanceModel, apps::AppModal, bluetooth::BluetoothModel,
    display::DisplayModel, mouse::MouseModal, multitasking::MultitaskingModel,
    network::NetworkModel, notifications::NotificationsModel, power::PowerModel,
    privacyandsecurity::PrivacyAndSecurityModel, rebuild::rebuild_dialog::RebuildInput,
    search::SearchModal, sharing::SharingModel, sound::SoundModel, system::SystemPageModel,
    wellbeing::WellbeingModel, wifi::WifiModel,
};
use crate::utils::modules::load::LoadOutput;
use crate::utils::state;
use crate::{
    config::{APP_ID, PROFILE},
    ui::rebuild::rebuild_dialog::{RebuildInit, RebuildModel},
};

use std::{convert::identity, fs};

pub struct App {
    stack: adw::ViewStack,
    wifi: AsyncController<WifiModel>,
    #[allow(dead_code)]
    network: Controller<NetworkModel>,
    #[allow(dead_code)]
    bluetooth: Controller<BluetoothModel>,
    #[allow(dead_code)]
    display: Controller<DisplayModel>,
    appearance: AsyncController<AppearanceModel>,
    #[allow(dead_code)]
    sound: Controller<SoundModel>,
    power: Controller<PowerModel>,
    #[allow(dead_code)]
    multitasking: Controller<MultitaskingModel>,
    apps: Controller<AppModal>,
    notifications: Controller<NotificationsModel>,
    #[allow(dead_code)]
    search: Controller<SearchModal>,
    #[allow(dead_code)]
    accounts: Controller<AccountsModel>,
    #[allow(dead_code)]
    sharing: Controller<SharingModel>,
    #[allow(dead_code)]
    wellbeing: Controller<WellbeingModel>,
    mouse: Controller<MouseModal>,
    #[allow(dead_code)]
    accessibility: Controller<AccessibilityModel>,
    #[allow(dead_code)]
    privacyandsecurity: Controller<PrivacyAndSecurityModel>,
    system: Controller<SystemPageModel>,

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
    Rebuild(String, String), // single line nix argument and value
    Reload,
    Quit,
}

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
    }
    view! {
    #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

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
                    set_title: &gettext("Settings"),
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            pack_end = &gtk::MenuButton {
                                set_icon_name: "open-menu-symbolic",
                                set_menu_model: Some(&primary_menu),
                            }
                        },
                        #[wrap(Some)]
                        set_content = &adw::ViewSwitcherSidebar {
                            set_stack: Some(&model.stack),
                        },
                    },
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        set_content: Some(&model.stack),
                    }
                },
            },
            add_breakpoint = bp_with_setters(
                adw::Breakpoint::new(
                    adw::BreakpointCondition::new_length(
                        adw::BreakpointConditionLengthType::MaxWidth,
                        600.0,
                        adw::LengthUnit::Px,
                    )
                ),
                &[
                (&split_view, "collapsed", true),
                ]
            ),
        },
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
        let notifications = NotificationsModel::builder().launch(()).detach();
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
        let mouse = MouseModal::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let accessibility = AccessibilityModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let privacyandsecurity = PrivacyAndSecurityModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let system = SystemPageModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let rebuild_dialog = RebuildModel::builder()
            .transient_for(&root)
            .launch(RebuildInit {
                flakepath,
                // modulepath,
                generations: config.generations,
            })
            .forward(sender.input_sender(), identity);

        let mut model = App {
            stack: adw::ViewStack::new(),
            wifi,
            network,
            bluetooth,
            display,
            appearance,
            sound,
            power,
            multitasking,
            apps,
            notifications,
            search,
            accounts,
            sharing,
            wellbeing,
            mouse,
            accessibility,
            privacyandsecurity,
            system,

            config,
            rebuild_dialog,
            // modified_config: HashMap::new(),
        };

        relm4::view! {
          view_stack = &adw::ViewStack {
              add_titled_with_icon: (model.wifi.widget(), Some("wifi"), "Wi-Fi", "network-wireless-symbolic"),
              // add_titled_with_icon: (model.network.widget(), Some("network"), "Network", "org.gnome.Settings-network-symbolic"),
              // add_titled_with_icon: (model.bluetooth.widget(), Some("bluetooth"), "Bluetooth", "org.gnome.Settings-bluetooth-symbolic"),
              add_titled_with_icon: (model.display.widget(), Some("display"), "Display", "org.gnome.Settings-display-symbolic"),
              add_titled_with_icon: (model.appearance.widget(), Some("appearance"), "Appearance", "org.gnome.Settings-appearance-symbolic"),
              // add_titled_with_icon: (model.sound.widget(), Some("sound"), "Sound", "org.gnome.Settings-sound-symbolic"),
              add_titled_with_icon: (model.power.widget(), Some("power"), "Power", "org.gnome.Settings-power-symbolic"),
              // add_titled_with_icon: (multitasking.widget(), Some("multitasking"), "Multitasking", "org.gnome.Settings-multitasking-symbolic"),
              add_titled_with_icon: (model.apps.widget(), Some("apps"), "Apps", "org.gnome.Settings-applications-symbolic"),
              add_titled_with_icon: (model.notifications.widget(), Some("notifications"), "Notifications", "org.gnome.Settings-notifications-symbolic"),
              // add_titled_with_icon: (search.widget(), Some("search"), "Search", "org.gnome.Settings-search-symbolic"),
              // add_titled_with_icon: (accounts.widget(), Some("accounts"), "Online Accounts", "org.gnome.Settings-online-accounts-symbolic"),
              // add_titled_with_icon: (sharing.widget(), Some("sharing"), "Sharing", "org.gnome.Settings-sharing-symbolic"),
              // add_titled_with_icon: (wellbeing.widget(), Some("wellbeing"), "Wellbeing", "org.gnome.Settings-wellbeing-symbolic"),
              add_titled_with_icon: (model.mouse.widget(), Some("mouse"), "Mouse and Touchpad", "input-mouse-symbolic"),
              // add_titled_with_icon: (accessibility.widget(), Some("accessibility"), "Acccesibility", "org.gnome.Settings-accessibility-symbolic"),
              // add_titled_with_icon: (privacyandsecurity.widget(), Some("privacyandsecurity"), "Privacy and Security", "org.gnome.Settings-privacy-symbolic"),
              add_titled_with_icon: (model.system.widget(), Some("system"), "System", "org.gnome.Settings-system-symbolic"),
              set_vhomogeneous: false,
              set_hhomogeneous: false,
          }
        }
        state::get_state()
            .and_then(|state| state.page)
            .map(|page| view_stack.set_visible_child_name(&page.value()));
        model.stack = view_stack;
        let display_stack = model.stack.page(model.display.widget());
        display_stack.set_starts_section(true);

        let apps_stack = model.stack.page(model.apps.widget());
        apps_stack.set_starts_section(true);

        let mouse_stack = model.stack.page(model.mouse.widget());
        mouse_stack.set_starts_section(true);

        let widgets = view_output!();
        model.stack.connect_visible_child_notify({
            let split_view = widgets.split_view.clone();
            move |_| {
                split_view.set_show_content(true);
            }
        });

        model.stack.connect_visible_child_name_notify(|stack| {
            stack
                .visible_child_name()
                .map(|s| s.to_string())
                .and_then(|name| state::Page::from_str(&name))
                .map(|page| state::update_state(|state| state.page = Some(page)));
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
            AppMsg::Rebuild(argument, value) => {
                // path to be written arg and val usually inside configuration.nix
                let configuration_nix: String = self.config.systemconfig.clone().unwrap();

                // String type readed file. e.x: {}, "{...}:\n{\n  i18n.defaultLocale..
                let full_config_string = fs::read_to_string(&configuration_nix)
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
                self.rebuild_dialog
                    .emit(RebuildInput::Rebuild(output.to_owned(), configuration_nix))
            }
            AppMsg::Reload => {}
            AppMsg::Quit => main_application().quit(),
        }
    }
    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets
            .save_window_size()
            .expect("can not save window size.............");
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

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
