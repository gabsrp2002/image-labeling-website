use sea_orm::*;
use crate::entity::admin::{Entity as Admin, Model as AdminModel, ActiveModel as AdminActiveModel};

pub struct AdminRepository;

impl AdminRepository {
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<AdminModel>, DbErr> {
        Admin::find()
            .filter(crate::entity::admin::Column::Username.eq(username))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        username: String,
        password_hash: String,
    ) -> Result<AdminModel, DbErr> {
        let admin = AdminActiveModel {
            username: Set(username),
            password_hash: Set(password_hash),
            ..Default::default()
        };
        
        admin.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<AdminModel>, DbErr> {
        Admin::find_by_id(id).one(db).await
    }

    pub async fn update_password(
        db: &DatabaseConnection,
        id: i32,
        new_password_hash: String,
    ) -> Result<AdminModel, DbErr> {
        let admin = Admin::find_by_id(id).one(db).await?;
        match admin {
            Some(admin) => {
                let mut admin: AdminActiveModel = admin.into();
                admin.password_hash = Set(new_password_hash);
                admin.update(db).await
            }
            None => Err(DbErr::RecordNotFound("Admin not found".to_string())),
        }
    }
}
