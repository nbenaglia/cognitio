# cognitio

This project is a RESTful backend to store and manage the level of knowledge of a group of people.

The level is a self-evaluation done by the subject using a range of values (corresponding to the points in parenthesis):

- guru        (10)
- expert       (7)
- tourist      (5)
- newbie       (3)
- zero-kelvin  (0)

[TODO]

## Development

A `docker-compose.yaml` file defines all the infrastructure you need.

In order to develop you first need:

- install `diesel` with `cargo install diesel_cli --no-default-features --features postgres`
- a running database with `docker-compose database up`

then you can run your build and test it.

To run all environment defined in docker-compose.yaml:

- from root folder `docker-compose -f docker/docker-compose.yaml --force-rm --no-cache up`

Then you can apply diesel scripts with:

- `diesel setup`
- `diesel migration run`

## Getting a token from Keycloak

 `export TOKEN=$(curl --request POST --url http://localhost:9090/auth/realms/projects/protocol/openid-connect/token -H 'Content-Type: application/x-www-form-urlencoded' -d client_id=cognitio -d client_secret=9da282d8-6ff1-4f07-bf09-46badc8af396 -d grant_type=client_credentials -d scope=email| jq -r .access_token)`

`curl 127.0.0.1:8080/technologies/ -v -H "Authorization: Bearer $TOKEN"`
