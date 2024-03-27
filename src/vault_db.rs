#![allow(dead_code)]
use libsql::Builder;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PasswordEntry {
    website: String,
    username: String,
    password: String,
}

pub struct Vault {
    name: String,
    encoding: String,
    conn: Option<libsql::Connection>,
}

impl Vault {
    pub async fn new(name: &str) -> Result<Vault, libsql::Error> {
        let mut v = Vault {
            name: name.to_string(),
            encoding: "".to_string(),
            conn: None,
        };
        match v.connect_local().await {
            Ok(conn) => v.conn = Some(conn),
            Err(e) => return Err(e),
        }
        Ok(v)
    }

    async fn connect_local(&self) -> libsql::Result<libsql::Connection> {
        let db = Builder::new_local("database/local.db").build().await?;
        db.connect()
    }

    // async fn connect_remote(&self) -> libsql::Result<libsql::Connection> {
    // let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    // let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    // let db = Builder::new_remote(&url, &token).build().await?;
    // db.connect()
    // }

    pub async fn print_passwords(&self) -> Result<(), libsql::Error> {
        if let Some(ref conn) = self.conn {
            let mut rows = conn.query("SELECT * FROM entries", ()).await?;
            while let Some(row) = rows.next().await? {
                println!("{:?}", row);
            }
        }
        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<PasswordEntry>, libsql::Error> {
        let mut entries = Vec::new();
        if let Some(ref conn) = self.conn {
            let mut rows = conn.query("SELECT * FROM entries", ()).await?;
            while let Some(row) = rows.next().await? {
                entries.push(libsql::de::from_row::<PasswordEntry>(&row).unwrap());
            }
        }
        Ok(entries)
    }

    pub async fn new_entry(
        &mut self,
        website: &str,
        username: &str,
        password: &str,
    ) -> Result<u64, libsql::Error> {
        if let Some(ref conn) = self.conn {
            return conn
                .execute(
                    "INSERT INTO entries (website, username, password) VALUES (?, ?, ?)",
                    [website, username, password],
                )
                .await;
        }
        Ok(0)
    }

    pub async fn get_entry(&self, website: &str) -> libsql::Result<Vec<PasswordEntry>> {
        let mut entries = Vec::new();
        if let Some(ref conn) = self.conn {
            let mut rows = conn
                .query("SELECT * FROM entries WHERE website = ? ", [website])
                .await?;
            while let Some(row) = rows.next().await? {
                entries.push(libsql::de::from_row::<PasswordEntry>(&row).unwrap());
            }
        }
        Ok(entries)
    }
}
