use crate::models::{NewUser, User};
use diesel::pg::PgConnection;
use uuid::Uuid;
use diesel::prelude::*;
use crate::models::UpdateUser;

pub fn get_or_create_user(
    conn: &mut PgConnection,
    sub: String,
    username: String,
    user_email: String,
    picture: String,
) -> User {
    use crate::schema::users;
    use crate::schema::users::dsl::*;

    users
        .filter(guid.eq(sub.clone()))
        .select((id, name, email, phone, avatar))
        .first::<User>(conn)
        .unwrap_or_else(|_| {
            let values = NewUser {
                guid: sub,
                name: username,
                email: user_email,
                avatar: picture,
            };
            diesel::insert_into(users::table)
                .values(&values)
                .returning((id, name, email, phone, avatar))
                .get_result(conn)
                .expect("Error saving new user")
        })
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> User {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .select((id, name, email, phone, avatar))
        .first::<User>(conn)
        .unwrap_or_else(|_| {
            panic!("Error getting user by id")
        })
}

pub fn update_user(
    conn: &mut PgConnection,
    user_id: Uuid,
    user_data: UpdateUser,
) -> User {
    use crate::schema::users::dsl::*;

    diesel::update(users.find(user_id))
        .set(user_data)
        .returning((id, name, email, phone, avatar))
        .get_result::<User>(conn)
        .expect("Error updating user")
}