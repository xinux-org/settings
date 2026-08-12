use clap::{Command, arg, command, value_parser};
use gettextrs::{LocaleCategory, gettext};
use relm4::{
    RelmApp,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::{self, gio, glib, prelude::ApplicationExt},
    main_application,
};
use settings::{
    config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE},
    ui::window::{App, AppInit},
    utils::{
        modules::load::load,
        state::{self, Page},
    },
};
use tracing::error;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn setup_locale() {
    // setup gettext
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
    glib::set_application_name(&gettext("Xinux settings"));

    // setup resources
    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);
    let data = res
        .lookup_data(
            "/uz/xinux/Settings/style.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    relm4::set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());
}

fn main() {
    // Enable logging
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();
    
    let matches = command!()
        .subcommand(
            Command::new("open").about("Page to open").arg(
                arg!([page])
                    .value_parser(value_parser!(Page))
                    .required(true),
            ),
        )
        .get_matches();
    if let Some(("open", sub_matches)) = matches.subcommand()
        && let Some(page) = sub_matches.get_one::<Page>("page")
    {
        state::update_state(|state| state.page = Some(page.clone()));
    };

    gtk::init().unwrap();
    gtk::Window::set_default_icon_name(APP_ID);
    setup_locale();

    let app = main_application();
    app.set_resource_base_path(Some("/uz/xinux/Settings/"));
    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };

    let mut actions = RelmActionGroup::<AppActionGroup>::new();
    actions.add_action(quit_action);
    actions.register_for_main_application();
    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    let app = RelmApp::from_app(app);
    match load() {
        Ok(load) => app.run::<App>(AppInit { load }),
        Err(e) => {
            error!("Failed to load: {}", e);
            std::process::exit(1);
        }
    }
}
