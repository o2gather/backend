use crate::models::{
    Event, EventMember, EventMsg, EventWithMembers, NewEvent, NewEventMember, NewEventMsg, NewUser,
    UpdateEvent, UpdateUser, User, EventOwner
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

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
        .select(User::as_select())
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
                .returning(User::as_select())
                .get_result(conn)
                .expect("Error saving new user")
        })
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> User {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first::<User>(conn)
        .unwrap_or_else(|_| panic!("Error getting user by id"))
}

pub fn update_user(conn: &mut PgConnection, user_id: Uuid, user_data: UpdateUser) -> User {
    use crate::schema::users::dsl::*;

    diesel::update(users.find(user_id))
        .set(user_data)
        .returning(User::as_select())
        .get_result::<User>(conn)
        .expect("Error updating user")
}

pub fn create_event(conn: &mut PgConnection, event_data: NewEvent) -> EventWithMembers {
    use crate::schema::event_members;
    use crate::schema::events;
    use crate::schema::users;

    let event = diesel::insert_into(events::table)
        .values(&event_data)
        .returning(Event::as_select())
        .get_result::<Event>(conn)
        .expect("Error creating event");

    let members = event_members::table
        .filter(event_members::event_id.eq(event.id))
        .inner_join(users::table)
        .select((
            users::name,
            users::email,
            users::phone,
            event_members::amount,
        ))
        .load::<EventMember>(conn)
        .expect("Error getting event members");
    
    let owner = users::table
        .filter(users::id.eq(event.user_id))
        .select(EventOwner::as_select())
        .first::<EventOwner>(conn)
        .expect("Error getting event owner");

    let amount: i64 = members.iter().map(|m| m.amount).sum();
    let members_count = members.len() as i64;

    EventWithMembers {
        id: event.id,
        user_id: event.user_id,
        owner: owner,
        name: event.name,
        description: event.description,
        category: event.category,
        start_time: event.start_time,
        end_time: event.end_time,
        min_amount: event.min_amount,
        max_amount: event.max_amount,
        amount,
        established: event.established,
        members: Some(members),
        members_count: members_count,
    }
}

pub fn get_events(conn: &mut PgConnection) -> Vec<EventWithMembers> {
    use crate::schema::events;
    use crate::schema::event_members;
    use crate::schema::users;

    let event = events::table
        .order(events::start_time.asc())
        .then_order_by(events::end_time.asc())
        .select(Event::as_select())
        .load::<Event>(conn)
        .expect("Error getting events");

    event
        .into_iter()
        .map(|e| {
            let members = event_members::table
            .filter(event_members::event_id.eq(e.id))
            .inner_join(users::table)
            .select((
                users::name,
                users::email,
                users::phone,
                event_members::amount,
            ))
            .load::<EventMember>(conn)
            .expect("Error getting event members");

            let owner = users::table
            .filter(users::id.eq(e.user_id))
            .select(EventOwner::as_select())
            .first::<EventOwner>(conn)
            .expect("Error getting event owner");
    
            let amount: i64 = members.iter().map(|m| m.amount).sum();

            EventWithMembers {
                id: e.id,
                user_id: e.user_id,
                owner: owner,
                name: e.name,
                description: e.description,
                category: e.category,
                start_time: e.start_time,
                end_time: e.end_time,
                min_amount: e.min_amount,
                max_amount: e.max_amount,
                amount,
                established: e.established,
                members: None,
                members_count: members.len() as i64,
            }
        })
        .collect()
}

pub fn get_event_by_id(
    conn: &mut PgConnection,
    event_id: Uuid,
    user_id: Uuid,
) -> Option<EventWithMembers> {
    use crate::schema::event_members;
    use crate::schema::events;
    use crate::schema::users;

    let ret = events::table
        .filter(events::id.eq(event_id))
        .select(Event::as_select())
        .first::<Event>(conn);
    let event: Event;
    match ret {
        Ok(e) => {
            event = e;
        }
        Err(_) => {
            return None;
        }
    }
    let owner = users::table
        .filter(users::id.eq(event.user_id))
        .select(EventOwner::as_select())
        .first::<EventOwner>(conn)
        .expect("Error getting event owner");

    let mut data = EventWithMembers {
        id: event.id,
        user_id: event.user_id,
        owner: owner,
        name: event.name,
        description: event.description,
        category: event.category,
        start_time: event.start_time,
        end_time: event.end_time,
        min_amount: event.min_amount,
        max_amount: event.max_amount,
        amount: 0,
        established: event.established,
        members: None,
        members_count: 0,
    };

    let members = event_members::table
            .filter(event_members::event_id.eq(event.id))
            .inner_join(users::table)
            .select((
                users::name,
                users::email,
                users::phone,
                event_members::amount,
            ))
            .load::<EventMember>(conn)
            .expect("Error getting event members");
    
    let amount: i64 = members.iter().map(|m| m.amount).sum();

    data.members_count = members.len() as i64;
    if event.user_id == user_id {
        data.members = Some(members);
    }
    data.amount = amount;

    Some(data)
}

pub fn get_event_members(conn: &mut PgConnection, event_id: Uuid) -> Vec<Uuid> {
    use crate::schema::event_members;

    event_members::table
        .filter(event_members::event_id.eq(event_id))
        .select(event_members::user_id)
        .load::<Uuid>(conn)
        .expect("Error getting event members")
}

pub fn update_event(
    conn: &mut PgConnection,
    event_id: Uuid,
    event_data: UpdateEvent,
) -> EventWithMembers {
    use crate::schema::event_members;
    use crate::schema::events;
    use crate::schema::users;

    let event = diesel::update(events::table.find(event_id))
        .set(event_data)
        .returning(Event::as_select())
        .get_result::<Event>(conn)
        .expect("Error updating event");

    let members = event_members::table
        .filter(event_members::event_id.eq(event.id))
        .inner_join(users::table)
        .select((
            users::name,
            users::email,
            users::phone,
            event_members::amount,
        ))
        .load::<EventMember>(conn)
        .expect("Error getting event members");

    let amount: i64 = members.iter().map(|m| m.amount).sum();
    let members_count = members.len() as i64;
    let owner = users::table
        .filter(users::id.eq(event.user_id))
        .select(EventOwner::as_select())
        .first::<EventOwner>(conn)
        .expect("Error getting event owner");

    EventWithMembers {
        id: event.id,
        user_id: event.user_id,
        owner: owner,
        name: event.name,
        description: event.description,
        category: event.category,
        start_time: event.start_time,
        end_time: event.end_time,
        min_amount: event.min_amount,
        max_amount: event.max_amount,
        amount,
        established: event.established,
        members: Some(members),
        members_count: members_count,
    }
}

pub fn delete_event(conn: &mut PgConnection, event_id: Uuid) -> bool {
    use crate::schema::events;

    diesel::delete(events::table.find(event_id))
        .execute(conn)
        .is_ok()
}

pub fn get_events_by_user_id(conn: &mut PgConnection, user_id: Uuid) -> Vec<EventWithMembers> {
    use crate::schema::event_members;
    use crate::schema::events;
    use crate::schema::users;

    let event1: Vec<Event> = events::table
        .filter(events::user_id.eq(user_id))
        .order(events::start_time.asc())
        .then_order_by(events::end_time.asc())
        .select(Event::as_select())
        .load::<Event>(conn)
        .expect("Error getting events");

    let event2: Vec<Event> = event_members::table
        .filter(event_members::user_id.eq(user_id))
        .inner_join(events::table)
        .select(Event::as_select())
        .load::<Event>(conn)
        .expect("Error getting events");

    

    let mut event: Vec<Event> = event1;
    event.extend(event2);
    event
        .into_iter()
        .map(|e| {
            let members = event_members::table
            .filter(event_members::event_id.eq(e.id))
            .inner_join(users::table)
            .select((
                users::name,
                users::email,
                users::phone,
                event_members::amount,
            ))
            .load::<EventMember>(conn)
            .expect("Error getting event members");
            let owner = users::table
            .filter(users::id.eq(e.user_id))
            .select(EventOwner::as_select())
            .first::<EventOwner>(conn)
            .expect("Error getting event owner");
    
            let amount: i64 = members.iter().map(|m| m.amount).sum();
            let mut data = EventWithMembers {
                id: e.id,
                user_id: e.user_id,
                owner: owner,
                name: e.name,
                description: e.description,
                category: e.category,
                start_time: e.start_time,
                end_time: e.end_time,
                min_amount: e.min_amount,
                max_amount: e.max_amount,
                amount,
                established: e.established,
                members: None,
                members_count: members.len() as i64,
            };
            if e.user_id == user_id {
                data.members = Some(members);
            }

            data
        })
        .collect()
}

pub fn create_event_member(
    conn: &mut PgConnection,
    event_member_data: NewEventMember,
) -> Result<(), Error> {
    use crate::schema::event_members;
    use crate::schema::events;

    conn.transaction::<(), _, _>(|conn| {
        diesel::insert_into(event_members::table)
            .values(&event_member_data)
            .on_conflict((event_members::event_id, event_members::user_id))
            .do_update()
            .set(event_members::amount.eq(event_member_data.amount))
            .execute(conn)?;

        let event: Event = events::table
            .filter(events::id.eq(event_member_data.event_id))
            .select(Event::as_select())
            .first::<Event>(conn)?;

        let total_amount: Vec<i64> = event_members::table
            .filter(event_members::event_id.eq(event.id))
            .select(event_members::amount)
            .load(conn)?;

        let total_amount: i64 = total_amount.iter().sum();

        if total_amount > event.max_amount {
            Err(Error::RollbackTransaction)
        } else {
            Ok(())
        }
    })
}

pub fn delete_event_member(conn: &mut PgConnection, event_id: Uuid, user_id: Uuid) -> bool {
    use crate::schema::event_members;

    diesel::delete(
        event_members::table
            .filter(event_members::event_id.eq(event_id))
            .filter(event_members::user_id.eq(user_id)),
    )
    .execute(conn)
    .is_ok()
}

pub fn create_event_msg(
    conn: &mut PgConnection,
    event_msg_data: NewEventMsg,
) -> Result<Vec<EventMsg>, Error> {
    use crate::schema::event_comments;
    use crate::schema::users;

    diesel::insert_into(event_comments::table)
        .values(&event_msg_data)
        .execute(conn)?;
    event_comments::table
        .filter(event_comments::event_id.eq(event_msg_data.event_id))
        .inner_join(users::table)
        .select(((users::name, users::avatar), event_comments::content, event_comments::created_at))
        .order(event_comments::created_at.asc())
        .load::<EventMsg>(conn)
}

pub fn get_event_msg_by_event_id(
    conn: &mut PgConnection,
    event_id: Uuid,
) -> Result<Vec<EventMsg>, Error> {
    use crate::schema::event_comments;
    use crate::schema::users;

    event_comments::table
        .filter(event_comments::event_id.eq(event_id))
        .inner_join(users::table)
        .select(((users::name, users::avatar), event_comments::content, event_comments::created_at))
        .order(event_comments::created_at.asc())
        .load::<EventMsg>(conn)
}

pub fn get_categories(conn: &mut PgConnection) -> Vec<String> {
    use crate::schema::events;
    
    events::table
        .select(events::category)
        .distinct()
        .load::<String>(conn)
        .expect("Error getting categories")
}