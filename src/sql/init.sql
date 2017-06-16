create table if not exists subscription (
    id integer primary key autoincrement,
    url text not null,
    label text not null, 
    created_at timestamp default current_timestamp,
    last_build_date text,
    unique(url)
);

create table if not exists podcast (
    id integer primary key autoincrement,
    subscription_id integer not null,
    url text not null,
    filename text not null,
    title text not null,
    content_text text not null,
    downloaded integer default 0,
    downloaded_at timestamp,
    created_at timestamp default current_timestamp,
    unique(url)
);

create table if not exists config (
    key text not null,
    value text,
    unique(key)
);
