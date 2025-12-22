use crate::RelmActionGroup;
use relm4::*;

use relm4::actions::RelmAction;

use adw::prelude::*;
use gtk::{gio, glib};

use crate::config::{APP_ID, PROFILE};
use crate::modals::{about::AboutDialog, counter::CounterModel, toggler::TogglerModel};
use std::convert::identity;

pub(super) struct App {
    _counter: Controller<CounterModel>,
    _toggler: Controller<TogglerModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = (u8, bool);
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About GTK Rust Template" => AboutAction,
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

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/net/bleur/GtkRustTemplate/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

                #[name(split_view)]
                adw::NavigationSplitView {
                  #[wrap(Some)]
                  set_sidebar = &adw::NavigationPage {
                      set_title: "Sidebar",

                      #[wrap(Some)]
                      set_child = &adw::ToolbarView {
                          add_top_bar = &adw::HeaderBar {},

                          #[wrap(Some)]
                          set_content = &gtk::StackSidebar {
                              set_stack: &stack,
                          },
                      },
                  },

                  #[wrap(Some)]
                  set_content = &adw::NavigationPage {
                      set_title: "Content",

                      #[wrap(Some)]
                      set_child = &adw::ToolbarView {
                          add_top_bar = &adw::HeaderBar {},
                          set_content: Some(&stack),
                      }
                  },
                },

                add_breakpoint = bp_with_setters(
                    adw::Breakpoint::new(
                        adw::BreakpointCondition::new_length(
                            adw::BreakpointConditionLengthType::MaxWidth,
                            400.0,
                            adw::LengthUnit::Sp,
                        )
                    ),
                    &[(&split_view, "collapsed", true)]
                ),
        },
        stack = &gtk::Stack {
            add_titled: (counter.widget(), None, "Counter"),
            add_titled: (toggler.widget(), None, "Toggle"),
            add_titled: (toggler.widget(), None, "aa"),
            add_titled: (toggler.widget(), None, "aaaa"),
            add_titled: (toggler.widget(), None, "Togglwwwe"),
            set_vhomogeneous: false,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counter = CounterModel::builder()
            .launch(init.0)
            .forward(sender.input_sender(), identity);
        let toggler = TogglerModel::builder()
            .launch(init.1)
            .forward(sender.input_sender(), identity);

        let widgets = view_output!();
        
        let model = App {
            _counter: counter,
            _toggler: toggler,
        };

        
        widgets.stack.connect_visible_child_notify({
            let split_view = widgets.split_view.clone();
            move |_| {
                split_view.set_show_content(true);
            }
        });
        
        

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
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
