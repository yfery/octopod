create table if not exists subscription (
    id integer primary key autoincrement not null,
    url text not null,
    label text, 
    last_build_date text,
    created_at timestamp default current_timestamp not null,
    unique(url)
);

create table if not exists podcast (
    id integer primary key autoincrement not null,
    subscription_id integer not null,
    url text not null,
    filename text not null,
    title text not null,
    content_text text not null,
    downloaded integer default 0 not null,
    downloaded_at timestamp,
    created_at timestamp default current_timestamp not null,
    unique(url)
);

create table if not exists config (
    key text primary key not null,
    value text,
    unique(key)
);
