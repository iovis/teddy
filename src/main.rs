use std::path::PathBuf;

use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut db = open(db_path()).await?;
    let notes = get_notes(&mut db).await?;

    for note in notes {
        println!("id: {}", note.id);
        println!("title: {}", note.title);
        println!("body: {}", note.body);
    }

    Ok(())
}

async fn open(path: PathBuf) -> Result<SqliteConnection, sqlx::Error> {
    let path = path.to_str().unwrap();
    SqliteConnection::connect(path).await
}

fn db_path() -> PathBuf {
    // TODO: not use a deprecated function
    let path = std::env::home_dir().unwrap();
    path.join(
        "Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear/Application Data/database.sqlite",
    )
}

#[derive(Debug, sqlx::FromRow)]
struct Note {
    id: String,
    title: String,
    body: String,
}

async fn get_notes(db: &mut SqliteConnection) -> Result<Vec<Note>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT
            ZUNIQUEIDENTIFIER as id,
            ZTITLE as title,
            ZTEXT as body
        FROM ZSFNOTE
        LIMIT 1
        OFFSET 3
        "#,
    )
    .fetch_all(db)
    .await
}
