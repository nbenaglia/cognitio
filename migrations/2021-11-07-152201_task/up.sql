CREATE TABLE technologies
(
    id          SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL
);

INSERT INTO technologies (id, description) values (1, 'linux');
