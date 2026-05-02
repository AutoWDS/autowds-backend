--- create database autowds;
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
    credits int not null default 100,
    invite_code varchar(20) unique not null,
    invited_by bigint null,
    email_subscribed boolean not null default true,
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
    rule jsonb not null,
    data jsonb default null,
    job_id uuid default null
);
create index idx_scraper_task_user_id_name_created on scraper_task(user_id, name, created);
--- task_instance
create sequence if not exists seq_task_instance;
create type instance_status as enum ('running', 'success', 'failed');
create table task_instance (
    id bigint primary key default nextval('seq_task_instance'),
    task_id bigint not null references scraper_task(id),
    created timestamp not null default current_timestamp,
    modified timestamp not null default current_timestamp,
    status instance_status not null,
    data_count int not null default 0,
    log_key varchar(500) null,
    error_message text null
);
create index idx_task_instance_task_id_created on task_instance(task_id, created desc);
--- credit_log
create sequence if not exists seq_credit_log;
create type credit_operation as enum ('REGISTER', 'INVITE', 'EXPORT', 'ADMIN_ADJUST', 'CHECK_IN');
create table if not exists credit_log (
    id bigint primary key default nextval('seq_credit_log'),
    created timestamp not null,
    user_id bigint not null,
    operation credit_operation not null,
    amount int not null,
    balance int not null,
    description varchar(200) null,
    related_user_id bigint null
);
create index idx_credit_log_user_id_created on credit_log(user_id, created desc);

-- 创建订单级别枚举类型
create type order_level as enum ('monthly', 'annual');

-- 创建支付来源枚举类型
create type pay_from as enum ('alipay', 'wechat', 'paddle');

-- 创建订单状态枚举类型
create type order_status as enum ('created', 'paid', 'closed');

-- 支付订单表
create table if not exists pay_order (
    id bigserial primary key,
    user_id bigint not null,
    level order_level not null,
    edition product_edition not null,
    pay_from pay_from not null,
    status order_status not null default 'created',
    created timestamp not null default current_timestamp,
    modified timestamp not null default current_timestamp,
    confirm timestamp null,
    resp jsonb null
);

create index idx_pay_order_user_id on pay_order(user_id);
create index idx_pay_order_status on pay_order(status);
create index idx_pay_order_created on pay_order(created);
create index idx_pay_order_confirm on pay_order(confirm);
