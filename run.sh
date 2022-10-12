docker network create todo-app
docker-compose up -d
diesel migration run
docker-compose exec rust-app cargo run -v