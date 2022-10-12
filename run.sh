docker network create todo-app
docker-compose up -d
docker-compose exec rust-app diesel migration run
docker-compose exec rust-app cargo run -v