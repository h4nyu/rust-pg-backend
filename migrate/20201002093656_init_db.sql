-- +goose Up
CREATE EXTENSION "uuid-ossp";
CREATE TABLE users ( 
    id text NOT NULL PRIMARY KEY,
    name text NOT NULL, 
    created_at timestamp NOT NULL
);


-- +goose Down
DROP EXTENSION "uuid-ossp";
DROP TABLE users;
