// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "scoring_type"))]
    pub struct ScoringType;
}

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ScoringType;

    sessions (id) {
        id -> Uuid,
        name -> Text,
        location -> Nullable<Text>,
        date -> Date,
        owner_id -> Uuid,
        scoring_type -> ScoringType,
        should_use_victory_points -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        role_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        salt -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        username -> Text,
    }
}

diesel::joinable!(sessions -> users (owner_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    roles,
    sessions,
    user_roles,
    users,
);
