create table if not exists podcast (
id integer primary key autoincrement,
url text not null,
label text not null, 
created_at timestamp default current_timestamp
)
