use std::any::type_name;

use libsql::Builder;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db = Builder::new_local("database/local.db").build().await?;
    let conn = db.connect().unwrap();
    let rows = conn.query("SELECT * FROM food_stats;", ()).await.unwrap();
    // while let Some(row) = rows.next().await.unwrap() {
    //     println!("{:?}", row.column_count());
    //     println!("{:?}", row.get(0).unwrap());
    //     // let item: Item = Item {
    //     //     task: row.get(0).unwrap(),
    //     // };
    //     // items.push(item);
    // }
    Ok(())
}
