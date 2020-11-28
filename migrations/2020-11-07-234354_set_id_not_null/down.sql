-- This file should undo anything in `up.sql`
-- Your SQL goes here
create table users_dg_tmp
(
    id VARCHAR primary key,
    nama VARCHAR not null,
    email VARCHAR not null,
    password VARCHAR not null
);

insert into users_dg_tmp(id, nama, email, password) select id, nama, email, password from users;

drop table users;

alter table users_dg_tmp rename to users;

create unique index autoindex_users_email
    on users (email);