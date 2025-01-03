-- Add migration script here
alter table users add column username text not null unique;