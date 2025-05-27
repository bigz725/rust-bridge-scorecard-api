FROM postgres:16.5-bullseye
COPY init.sql /docker-entrypoint-initdb.d/