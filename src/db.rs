use crate::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
