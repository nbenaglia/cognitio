CREATE TABLE technologies
(
    id          SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL,
    created_at  TIMESTAMP
);

INSERT INTO technologies (id, description, created_at) values (1, 'linux', '2004-10-19 10:23:54');
