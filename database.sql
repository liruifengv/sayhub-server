drop table if exists todo_list;
drop table if exists todo_item;
drop table if exists a;

create table if not exists todo_list (
  id serial primary key,
  title varchar(255) not null
);

create table if not exists todo_item (
  id serial primary key,
  title varchar(255) not null,
  checked boolean not null default false,
  list_id integer not null,
  foreign key (list_id) references todo_list(id)
);

insert into todo_list (title) values ('list1'), ('list2'), ('list3');

insert into todo_item (title, list_id)
  values ('items1', 1), ('items2', 2), ('items3', 3);

create table if not exists a (id serial primary key, tag int[]);  

insert into a (tag) values (array[6,7,8,9]), (array[6,7,8,9]), (array[6,7,8,9]);