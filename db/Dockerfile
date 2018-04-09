# docker run --rm -d -e POSTGRES_PASSWORD=test -p 5432:5432 postgres

FROM postgres
# ADD addresses.csv /
# ADD users.csv /
# ADD init.sql /docker-entrypoint-initdb.d/init.sql
ADD data/ /var/lib/postgresql/data
