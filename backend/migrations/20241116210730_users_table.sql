-- Add migration script here
create table "users" (
    id uuid primary key default gen_random_uuid(),
    email text not null unique,
    password text not null,
    salt text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

create table "roles" (
    id uuid primary key default gen_random_uuid(),
    name text not null unique,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

create table "user_roles" (
    id uuid primary key default gen_random_uuid(),
    user_id uuid references users(id),
    role_id uuid references roles(id)
);