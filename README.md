# todo-app-benchmarks
<img src="https://img.shields.io/badge/build-passed-red?" /> <img src="https://img.shields.io/badge/Rust-1.64.0-red?logo=rust" /> <img src="https://img.shields.io/badge/Docker-20.10.12-blue?logo=docker" />
<br>
Todo application for benchmarking.<br>
Benchmarks are obtained for server counts 1-3.

## Feature
+ REST API
+ Rust ver.1.64.0
+ actix-web ver.4
+ AWS (EC2, ELB, RDS)
+ Docker

## BenchMark
### Graph
### Server counts 1
### Server counts 2
### Server counts 3

## Evidence

## Starting up node app
```terminal
# install git, docker
$ git clone https://github.com/AsadaGuitar/todo-app-benchmarks.git
$ cd todo-app-benchmarks
$ chmod 755 ./run.sh
$ ./run.sh
``` 

## Examples
```terminal
$ curl -X GET http://127.0.0.1:8080/mNXpMugRoTXI/task
{"id":"0b03b582-ca4c-4824-a05d-bf26ff5027fb","user_id":"mNXpMugRoTXI","tasks":[{"id":"sEUMhpeeNOyu","user_id":"mNXpMugRoTXI","title":"Reservations for Year-End Party","close":false,"create_at":"2022-10-11T19:42:42.633215Z","modify_at":null,"close_at":null}],"create_at":"2022-10-12T13:28:44.094761460Z","time_zone":"utc"}
$
$ curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy Coffee"}' http://127.0.0.1:8080/mNXpMugRoTXI/task/
{"id":"f6478699-16fe-44d1-a88f-fec9c2d05aeb","user_id":"mNXpMugRoTXI","task_id":"9975067a-863","title":"Buy Coffee","create_at":"2022-10-12T13:32:54.065204465Z","time_zone":"utc"}
$
$ curl -X PUT -H "Content-Type: application/json" -d '{"title":"Destroy Python"}' http://127.0.0.1:8080/mNXpMugRoTXI/task/9975067a-863
{"id":"3bfc9b67-21fe-4a88-8b19-9c091acfb92a","user_id":"mNXpMugRoTXI","task":{"id":"9975067a-863","user_id":"mNXpMugRoTXI","title":"Destroy Python","close":false,"create_at":"2022-10-12T13:32:54.065204Z","modify_at":"2022-10-12T13:35:42.311324Z","close_at":null},"create_at":"2022-10-12T13:35:42.313735792Z","time_zone":"utc"}
$
$ curl -X GET curl -X GET http://127.0.0.1:8080/mNXpMugRoTXI/task/9975067a-863
{"id":"abcc61ec-4ff7-4402-b182-0e40e163f026","user_id":"mNXpMugRoTXI","task":{"id":"9975067a-863","user_id":"mNXpMugRoTXI","title":"Destroy Python","close":false,"create_at":"2022-10-12T13:32:54.065204Z","modify_at":"2022-10-12T13:35:42.311324Z","close_at":null},"create_at":"2022-10-12T13:37:00.808359551Z","time_zone":"utc"}
$
$ curl -X DELETE http://127.0.0.1:8080/mNXpMugRoTXI/task/9975067a-863
{"id":"3b06bce3-8a94-413b-8f4f-6af3af172e3b","user_id":"mNXpMugRoTXI","task_id":"9975067a-863","create_at":"2022-10-12T13:37:51.243891463Z","time_zone":"utc"}
$
$ curl -X GET curl -X GET http://127.0.0.1:8080/mNXpMugRoTXI/task/9975067a-863
Not found task 9975067a-863
```