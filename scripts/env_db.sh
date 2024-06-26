# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"

#Check if a custom pasword name has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

#Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Check if custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

export PGPASSWORD="${DB_PASSWORD}"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
