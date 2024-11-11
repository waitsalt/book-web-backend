use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: i64,          // 用户ID
    pub user_name: String,     // 用户名
    pub user_password: String, // 密码
    pub picture: String,       // 头像
    pub email: String,         // 邮箱
    pub is_admin: bool,        // 是否为管理员
    pub is_delete: bool,       // 是否被删除
    pub create_time: String,   // 创建时间
}
