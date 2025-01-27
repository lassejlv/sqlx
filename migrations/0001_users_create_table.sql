CREATE TABLE users (
  id serial primary key,
  email text not null,
  password_hash text not null
);

CREATE UNIQUE INDEX users_email_idx ON users (email);
