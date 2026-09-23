use std::{cell::RefCell, rc::Rc};

use gettextrs::gettext;
use qrcodegen::{QrCode, QrCodeEcc};
use relm4::{
    adw::{self, prelude::*},
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self},
    *,
};

pub struct WifiQrDialog {
    ssid: String,
    password: String,
    has_password: bool,
    qr_data: Rc<RefCell<Option<QrCode>>>,
    drawing_area: gtk::DrawingArea,
    parent: gtk::Widget,
    dialog: adw::Dialog,
}

#[derive(Debug)]
pub enum WifiQrInput {
    Show {
        ssid: String,
        password: Option<String>,
        security: String,
    },
    Close,
}

fn escape_wifi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(c, '\\' | ';' | ',' | '"' | ':') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[relm4::component(pub, async)]
impl AsyncComponent for WifiQrDialog {
    type Init = gtk::Widget;
    type Input = WifiQrInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::Dialog {
            set_content_width: 350,
            set_content_height: 510,
            set_title: &gettext("Share Network"),
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    adw::PreferencesGroup {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            gtk::Box {
                                set_margin_bottom: 8,
                                set_halign: gtk::Align::Center,
                                #[name(drawing_area)]
                                gtk::DrawingArea {
                                    set_content_width: 200,
                                    set_content_height: 200,
                                },
                            },
                            gtk::Label {
                                add_css_class: "title-1",
                                set_text: &gettext("Scan to Connect"),
                                set_wrap: true,
                                set_justify: gtk::Justification::Center,
                            },
                        },
                    },
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            add_css_class: "property",
                            set_use_markup: false,
                            set_title: &gettext("Network Name"),
                            #[watch]
                            set_subtitle: &model.ssid,
                            set_subtitle_selectable: true,
                        },

                        adw::ActionRow {
                            add_css_class: "property",
                            set_use_markup: false,
                            set_title: &gettext("Password"),
                            #[watch]
                            set_subtitle: &model.password,
                            set_subtitle_selectable: true,
                            #[watch]
                            set_visible: model.has_password,
                        },
                    },
                },
            }
        }
    }

    async fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let qr_data: Rc<RefCell<Option<QrCode>>> = Rc::new(RefCell::new(None));

        let mut model = WifiQrDialog {
            ssid: String::new(),
            password: String::new(),
            has_password: false,
            qr_data: qr_data.clone(),
            drawing_area: gtk::DrawingArea::new(),
            parent,
            dialog: root.clone(),
        };

        let widgets = view_output!();

        let qr_ref = qr_data.clone();
        widgets
            .drawing_area
            .set_draw_func(move |_area, cr, width, height| {
                cr.set_source_rgb(1.0, 1.0, 1.0);
                let _ = cr.paint();

                let guard = qr_ref.borrow();
                if let Some(ref qr) = *guard {
                    let size = qr.size() as f64;
                    let quiet = 2.0_f64;
                    let total = size + 2.0 * quiet;
                    let module_px = (width.min(height) as f64) / total;
                    let offset_x = (width as f64 - total * module_px) / 2.0;
                    let offset_y = (height as f64 - total * module_px) / 2.0;

                    cr.set_source_rgb(0.0, 0.0, 0.0);
                    for y in 0..qr.size() {
                        for x in 0..qr.size() {
                            if qr.get_module(x, y) {
                                let px = offset_x + (x as f64 + quiet) * module_px;
                                let py = offset_y + (y as f64 + quiet) * module_px;
                                cr.rectangle(px, py, module_px, module_px);
                                let _ = cr.fill();
                            }
                        }
                    }
                }
            });
        model.drawing_area = widgets.drawing_area.clone();
        
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            WifiQrInput::Show {
                ssid,
                password,
                security,
            } => {
                let wifi_str = format!(
                    "WIFI:T:{};S:{};P:{};;",
                    security,
                    escape_wifi(&ssid),
                    escape_wifi(password.as_deref().unwrap_or("")),
                );
                *self.qr_data.borrow_mut() = QrCode::encode_text(&wifi_str, QrCodeEcc::Medium).ok();

                let pass = password.unwrap_or_default();
                self.has_password = !pass.is_empty();
                self.password = pass;
                self.ssid = ssid;
                self.drawing_area.queue_draw();

                self.dialog.present(Some(&self.parent));
            }
            WifiQrInput::Close => {
                self.dialog.close();
            }
        }
    }
}
