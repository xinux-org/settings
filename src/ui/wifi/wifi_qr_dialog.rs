use std::{cell::RefCell, rc::Rc};

use qrcodegen::{QrCode, QrCodeEcc};
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    adw::{self, prelude::*},
    gtk::{self},
};

pub struct WifiQrDialog {
    visible: bool,
    ssid: String,
    qr_data: Rc<RefCell<Option<QrCode>>>,
    drawing_area: gtk::DrawingArea,
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

#[relm4::component(pub)]
impl SimpleComponent for WifiQrDialog {
    type Init = ();
    type Input = WifiQrInput;
    type Output = ();

    view! {
        #[root]
        adw::Window {
            set_modal: true,
            set_resizable: false,
            set_default_width: 320,
            set_title: Some("Share Wi-Fi"),
            #[watch]
            set_visible: model.visible,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_top: 16,
                    set_margin_bottom: 24,
                    set_margin_start: 24,
                    set_margin_end: 24,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        add_css_class: "title-2",
                        #[watch]
                        set_text: &model.ssid,
                    },

                    #[name(drawing_area)]
                    gtk::DrawingArea {
                        set_content_width: 240,
                        set_content_height: 240,
                    },

                    gtk::Label {
                        set_text: "Scan to connect to this network",
                        add_css_class: "dim-label",
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let qr_data: Rc<RefCell<Option<QrCode>>> = Rc::new(RefCell::new(None));

        let mut model = WifiQrDialog {
            visible: false,
            ssid: String::new(),
            qr_data: qr_data.clone(),
            drawing_area: gtk::DrawingArea::new(),
        };

        let widgets = view_output!();

        model.drawing_area = widgets.drawing_area.clone();

        let qr_ref = qr_data.clone();
        widgets.drawing_area.set_draw_func(move |_area, cr, width, height| {
            // White background
            cr.set_source_rgb(1.0, 1.0, 1.0);
            let _ = cr.paint();

            let guard = qr_ref.borrow();
            if let Some(ref qr) = *guard {
                let size = qr.size() as f64;
                let quiet = 4.0_f64; // quiet zone in modules
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

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            WifiQrInput::Show { ssid, password, security } => {
                let wifi_str = format!(
                    "WIFI:T:{};S:{};P:{};;",
                    security,
                    escape_wifi(&ssid),
                    escape_wifi(password.as_deref().unwrap_or("")),
                );
                *self.qr_data.borrow_mut() =
                    QrCode::encode_text(&wifi_str, QrCodeEcc::Medium).ok();
                self.ssid = ssid;
                self.visible = true;
                self.drawing_area.queue_draw();
            }
            WifiQrInput::Close => {
                self.visible = false;
            }
        }
    }
}
