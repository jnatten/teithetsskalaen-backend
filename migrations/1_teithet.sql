create table "teithet" (
    id bigserial primary key,
    title text not null,
    description text not null,
    created_at timestamp not null default now()
);
