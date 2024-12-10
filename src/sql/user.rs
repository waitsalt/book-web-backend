use sqlx::{Pool, Postgres};

use crate::{
    model::user::User,
    util::{app_error::AppError, Result},
};

pub async fn query_user_name_is_exist(pool: &Pool<Postgres>, user_name: &str) -> Result<()> {
    let sql = "
    select 
        user_id
    from
        \"user\"
    where
        user_name = $1;";
    let affected_row = sqlx::query(sql)
        .bind(user_name)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserNameExist);
    }
    Ok(())
}

pub async fn query_user_email_is_exist(pool: &Pool<Postgres>, user_email: &str) -> Result<()> {
    let sql = "
        select 
            user_id
        from
            \"user\"
        where
            user_email = $1;";
    let affected_row = sqlx::query(sql)
        .bind(user_email)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserEmailExist);
    }
    Ok(())
}

pub async fn create_user(
    pool: &Pool<Postgres>,
    user_name: &str,
    user_password: &str,
    user_email: &str,
    avatar_url: &str,
) -> Result<()> {
    let sql = "
    insert into
        \"user\" (user_name, user_password, user_email, avatar_url)
    values
        ($1,$2,$3,$4);";
    let _affected_row = sqlx::query(sql)
        .bind(user_name)
        .bind(user_password)
        .bind(user_email)
        .bind(avatar_url)
        .execute(pool)
        .await
        .unwrap()
        // .map_err(|_| AppError::UserCreateFailure)?
        .rows_affected();
    Ok(())
}

pub async fn delete_user(pool: &Pool<Postgres>, user_id: &i32) -> Result<()> {
    let sql = "
    upadte
        \"user\"
    set
        user_status = 2
    where
        user_id = $1;";
    let _ = sqlx::query(sql).bind(user_id).execute(pool).await.unwrap();
    Ok(())
}

pub async fn get_all_user_info(pool: &Pool<Postgres>) -> Result<Vec<User>> {
    let sql = "
    select
        *
    from
        \"user\";";

    let users: Vec<User> = sqlx::query_as(sql).fetch_all(pool).await?;
    Ok(users)
}

pub async fn get_user_info_by_id(pool: &Pool<Postgres>, user_id: &i32) -> Result<User> {
    let sql = "
    select
        *
    from 
        \"user\"
    where
        user_id = $1;";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    match res {
        Some(user) => {
            return Ok(user);
        }
        None => {
            return Err(AppError::UserNotExist);
        }
    }
}

pub async fn get_user_info_by_name(pool: &Pool<Postgres>, user_name: &str) -> Result<User> {
    let sql = "
    select
        *
    from 
        \"user\"
    where
        user_name = $1;";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(&user_name)
        .fetch_optional(pool)
        .await?;
    match res {
        Some(user) => {
            return Ok(user);
        }
        None => {
            return Err(AppError::UserNotExist);
        }
    }
}

pub async fn update_base_info(
    pool: &Pool<Postgres>,
    user_id: &i32,
    avatar_url: &str,
    user_name: &str,
) -> Result<()> {
    let sql = "
    update
        \"user\"
    set
        avatar_url = $1
    and
        user_name = $2
    where
        user_id = $3";
    let _ = sqlx::query(sql)
        .bind(avatar_url)
        .bind(user_name)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_avatar_url(
    pool: &Pool<Postgres>,
    user_id: &i32,
    avatar_url: &str,
) -> Result<()> {
    let sql = "
    update
        \"user\"
    set
        avatar_url = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(avatar_url)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_email(pool: &Pool<Postgres>, user_id: &i32, user_email: &str) -> Result<()> {
    let sql = "
    update
        \"user\"
    set
        user_email = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(user_email)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_password(
    pool: &Pool<Postgres>,
    user_id: &i32,
    user_password: &str,
) -> Result<()> {
    let sql = "
    update
        \"user\"
    set
        user_password = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(user_password)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}
