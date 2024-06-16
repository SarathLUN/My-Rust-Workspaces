-- Your SQL goes here
create table events (
    id serial primary key ,
    name text not null,
    description text not null ,
    location text not null
);