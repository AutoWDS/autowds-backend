create database autowds;
--- user
create sequence if not exists seq_account_user;
create type product_edition as enum ('L0', 'L1', 'L2', 'L3');
create table if not exists account_user (
    id bigint primary key default nextval('seq_account_user'),
    created timestamp not null,
    modified timestamp not null,
    edition product_edition not null,
    name varchar(32) not null,
    email varchar(64) not null,
    passwd varchar(32) not null,
    locked boolean not null,
    last_login inet null,
    unique (email)
);
--- template
create sequence if not exists seq_template;
create type template_topic as enum (
    'SOCIAL_NETWORK',
    'RESEARCH_EDUCATION',
    'E_COMMERCE',
    'LOCAL_LIFE',
    'BIDDING',
    'MEDIA',
    'SEARCH_ENGINE',
    'OTHER'
);
create table if not exists task_template (
    id bigint primary key default nextval('seq_template'),
    created timestamp not null,
    modified timestamp not null,
    topic template_topic not null,
    edition product_edition not null,
    lang varchar(6) not null,
    fav_count int not null,
    name varchar(80) not null,
    detail varchar(200) not null,
    img varchar(200) not null,
    rule jsonb not null,
    data jsonb not null,
    params jsonb null
);
--- fav
create sequence if not exists seq_favorite;
create table if not exists favorite (
    id bigint primary key default nextval('seq_favorite'),
    created timestamp not null,
    user_id bigint not null,
    template_id bigint not null,
    unique (user_id, template_id)
);
--- scraper_task
create sequence seq_scraper_task;
create table scraper_task (
    id bigint primary key default nextval('seq_scraper_task'),
    created timestamp not null,
    modified timestamp not null,
    user_id bigint not null,
    deleted boolean not null,
    name varchar(60) not null,
    rule jsonb not null
);
create index idx_scraper_task_user_id_name_created on scraper_task(user_id, name, created);