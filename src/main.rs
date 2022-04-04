pub mod entities;
use entities::{game};
use sea_orm::{entity::*, error::*, Database, DbConn};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    env_logger::init();
    let db: DbConn = Database::connect("mysql://sea_usr:password@dbhost:3306/sea").await.unwrap();
    let new_game = game::ActiveModel {
        home_team_id: Set(1),
        away_team_id: Set(2),
        ..Default::default()
    };
    new_game.insert(&db).await?;
    Ok(())
}
