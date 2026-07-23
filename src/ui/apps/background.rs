use crate::ui::apps::permission_store::PermissionStoreProxyBlocking;
use zbus::blocking::Connection;
use zbus::Result;

const TABLE: &str = "background";
const ID: &str = "background";

pub struct BackgroundPermission {
    proxy: PermissionStoreProxyBlocking<'static>,
    app_id: String
}

impl BackgroundPermission {
    pub fn new(app_id: &str) -> Result<Self> {
        let conn = Connection::session()?;
        let proxy = PermissionStoreProxyBlocking::new(&conn)?;
        Ok(Self { proxy, app_id: app_id.to_string() })
    }

    pub fn is_allowed(&self) -> bool {
        match self.proxy.get_permission(TABLE, ID, &self.app_id){
            Ok(perms) => perms.iter().any(|p| p == "yes"),
            Err(_) => false
        }
    }

    pub fn set_allowed(&self, allow: bool) -> Result<()> {
        let value = if allow { "yes" } else { "no" };
        self.proxy
            .set_permission(TABLE, true, ID, &self.app_id, &[value])
    }
}
