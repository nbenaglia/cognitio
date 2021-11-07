# cognitio

This project is a RESTful backend to store and manage the level of knowledge of a group of people.

The level is a self-evaluation done by the subject using a range of values (and their points in parenthesis):

- guru        (10)
- expert       (7)
- tourist      (5)
- newbie       (3)
- zero-kelvin  (0)

[TODO]

## Development

A `docker-compose.yaml` file defines all the infrastructure you need.

In order to develop you first need a running database:

`docker-compose up database`
