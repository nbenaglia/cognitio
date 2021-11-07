CREATE TABLE technologies
(
    id          SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL,
    created_at  TIMESTAMP NOT NULL
);

INSERT INTO technologies (description, created_at) values ('linux', '2004-10-19 10:23:54');
