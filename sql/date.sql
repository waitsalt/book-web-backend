create table "user" (
    user_id serial primary key,
    user_name text not null unique,
    user_password text not null,
    user_email text not null unique,
    avatar_url text not null default '',
    level int2 not null default 0,
    status int2 not null default 0,
    identity int2 not null default 0,
    create_time timestamp with time zone not null default now(),
    update_time timestamp with time zone not null default now()
);

insert into
    public."user" (user_name, user_password, user_email, identity)
values
    ('root', '123456', 'root@root.com', 2);

create table author (
    author_id serial primary key,
    author_name text not null,
    platform text default '',
    create_time timestamp with time zone not null default now(),
    update_time timestamp with time zone not null default now()
);

create unique index author_index_author_name_platform on author (author_name, platform);

create table book (
    book_id serial primary key,
    book_name text not null,
    author_id int,
    author_name text,
    platform text default '',
    user_id int,
    user_name text,
    cover_url text default '',
    source_url text default '',
    book_class text default '',
    book_status text not null default '',
    book_tags text default '',
    book_desc text not null default '',
    latest_chapter_id int default 0,
    latest_chapter_name text default '书籍说明',
    create_time timestamp with time zone not null default now(),
    update_time timestamp with time zone not null default now(),
    foreign key (user_id) references "user" (user_id),
    foreign key (author_id) references author (author_id)
);

create unique index book_index_book_name on book (book_name, author_name, platform);

create table chapter (
    book_id int,
    book_name text,
    author_id int,
    author_name text,
    platform text default '',
    chapter_id int,
    chapter_name text,
    chapter_content text,
    roll_id int2 default 0,
    roll_name text default '正文',
    create_time timestamp with time zone not null default now(),
    update_time timestamp with time zone not null default now(),
    foreign key (book_id) references book (book_id),
    foreign key (author_id) references author (author_id),
    primary key (book_id, chapter_id)
);