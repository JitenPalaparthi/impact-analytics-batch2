docker run --name usersdb \
  -e POSTGRES_DB=usersdb \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 \
  -v "$(pwd)/init.sql:/docker-entrypoint-initdb.d/init.sql:ro" \
  -d postgres:16
