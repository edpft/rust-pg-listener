db-up:
    docker compose up -d db

db-down:
    docker compose stop db && docker compose rm db