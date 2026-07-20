mod permission_store;
use permission_store::PermissionStoreProxy;
use zbus::Connection;

const TABLE: &str = "background";
const ID: &str = "background";

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn = Connection::session().await?;
    let store = PermissionStoreProxy::new(&conn).await?;
    
    let v = store.get_permission(TABLE, ID, "org.gnome.Fractal").await?;
    println!("Fractal: {v:?}");
    
    store.set_permission(TABLE, true, ID, "org.gnome.Fractal", &["yes"]).await?;
    
    store.delete_permission(TABLE, ID, "org.gnome.Fractal").await?;
    
    Ok(())
}