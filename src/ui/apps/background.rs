use crate::ui::apps::permission_store::PermissionStoreProxyBlocking;
use zbus::blocking::Connection;

// const TABLE: &str = "background";
// const ID: &str = "background";

pub struct BackgroundPermission {
    proxy: PermissionStoreProxyBlocking<'static>,
    app_id: String
}

impl BackgroundPermission {
    pub fn new(app_id: &str) -> Option<Self> {
        let conn = Connection::session()
            .map_err(|e| eprintln!("Session bus ochilmadi: {e}"))
            .ok()?;
        let proxy = PermissionStoreProxyBlocking::new(&conn)
            .map_err(|e| eprintln!("PermissionStore proxy xato: {e}"))
            .ok()?;
        Some(Self { proxy, app_id: app_id.to_string() })
    }

    pub fn get_perm(&self) -> bool {
        match self.proxy.get_permission("background", "background", &self.app_id){
            Ok(perms) => perms.iter().any(|p| p == "no"),
            Err(_) => true
        }
    }

    pub fn set_allowed(&self, allow: bool) -> zbus::Result<()> {
        let value = if allow { "yes" } else { "no" };
        self.proxy
            .set_permission("background", true, "background", &self.app_id, &[value])
    }
}
