# Market API version 2.0
A rewrite of the Minecraft market API available [here](https://github.com/MinesoftCC/market-api). 

This rewrite makes use of Docker and `docker-compose` to create an easily deployable database and API.

## Changes from the previous version
- The API now instead communicates with a database rather than reading and writing to a JSON file in a data directory.

## How to run/deploy
There are a few prerequisites that need to be met before the following commands are ran.

Firstly, the following environment variable(s) need to be defined for the process to work:
- `DATABASE_HOST`
- `DATABASE_PORT`
- `MYSQL_ROOT_PASSWORD`
- `MYSQL_USER`
- `MYSQL_PASSWORD`

The following environment variables are optional to define:

- `HOST`: describes the host on which to host the API on (defaults to `0.0.0.0` if not specified).
- `PORT`: describes the port on which the API should use (defaults to `8000` if not specified).

The Docker commands should be ran as `root` (denoted by the `#` at the beginning of the commands) through `sudo`/`doas` or just by being the `root` user.

### Running just the API
`# docker-compose run api -d`

### Running just the database
`# docker-compose run db -d`

### Running both
`# docker-compose up -d`
