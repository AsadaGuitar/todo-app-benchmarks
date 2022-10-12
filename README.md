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
```bash
# install git, docker
$ git clone https://github.com/AsadaGuitar/todo-app-benchmarks.git
$ cd todo-app-benchmarks
$ chmod 755 ./run.sh && ./run.sh
``` 

## Examples
```bash
$ curl -X POST http://127.0.0.1:8080/user/create 
{"id":"6d635ffb-e1e2-4323-95a2-50cc82f16abe","user_id":"CFLBMHTMUZCG","create_at":"2022-10-12T20:46:39.075754878Z","time_zone":"utc"}
$
$ curl -X GET http://127.0.0.1:8080/CFLBMHTMUZCG/task
{"id":"b13e9dc4-846f-46b4-8190-c0ac3d7f944b","user_id":"CFLBMHTMUZCG","tasks":[],"create_at":"2022-10-12T20:46:52.970825135Z","time_zone":"utc"}
$
$ curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy Coffee"}' http://127.0.0.1:8080/CFLBMHTMUZCG/task
{"id":"87096a56-f2cf-4fad-aa29-f4a19f35d00c","user_id":"CFLBMHTMUZCG","task_id":"1444a869-998","title":"Buy Coffee","create_at":"2022-10-12T20:47:17.266957965Z","time_zone":"utc"}
$
$ curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy Rust instructional books"}' http://127.0.0.1:8080/CFLBMHTMUZCG/task
{"id":"9e2bc5cd-44ff-41a0-845f-ef03858647b9","user_id":"CFLBMHTMUZCG","task_id":"cea30ba5-d68","title":"Buy Rust instructional books","create_at":"2022-10-12T20:48:21.694525176Z","time_zone":"utc"}
$
$ curl -X GET http://127.0.0.1:8080/CFLBMHTMUZCG/task
{"id":"5fd6e0ae-ec46-4d78-9459-1f5c506b8d65","user_id":"CFLBMHTMUZCG","tasks":["1444a869-998","cea30ba5-d68"],"create_at":"2022-10-12T20:48:35.738578293Z","time_zone":"utc"}
$
$ curl -X GET http://127.0.0.1:8080/CFLBMHTMUZCG/task/1444a869-998
{"id":"20168c28-5b51-4bd8-a27a-bb99129b8d67","user_id":"CFLBMHTMUZCG","task":{"id":"1444a869-998","user_id":"CFLBMHTMUZCG","title":"Buy Coffee","close":false,"create_at":"2022-10-12T20:47:17.266957Z","modify_at":null,"close_at":null},"create_at":"2022-10-12T20:48:59.240836929Z","time_zone":"utc"}
$ 
$ curl -X PUT -H "Content-Type: application/json" -d '{"title":"Destroy Python"}' http://127.0.0.1:8080/CFLBMHTMUZCG/task/1444a869-998
{"id":"743e56eb-bb7c-44f0-a8f1-3b906668c23d","user_id":"CFLBMHTMUZCG","task":{"id":"1444a869-998","user_id":"CFLBMHTMUZCG","title":"Destroy Python","close":false,"create_at":"2022-10-12T20:47:17.266957Z","modify_at":"2022-10-12T20:49:28.274864Z","close_at":null},"create_at":"2022-10-12T20:49:28.278667387Z","time_zone":"utc"}
$
$ curl -X GET http://127.0.0.1:8080/CFLBMHTMUZCG/task/1444a869-998
{"id":"5f43ea57-0c48-4580-9e08-08a48522abe7","user_id":"CFLBMHTMUZCG","task":{"id":"1444a869-998","user_id":"CFLBMHTMUZCG","title":"Destroy Python","close":false,"create_at":"2022-10-12T20:47:17.266957Z","modify_at":"2022-10-12T20:49:28.274864Z","close_at":null},"create_at":"2022-10-12T20:51:05.091645626Z","time_zone":"utc"}
$
$ curl -X DELETE http://127.0.0.1:8080/CFLBMHTMUZCG/task/1444a869-998
{"id":"0e50bd29-1151-4ba5-87b3-61b61b15c595","user_id":"CFLBMHTMUZCG","task_id":"1444a869-998","create_at":"2022-10-12T20:51:38.843966878Z","time_zone":"utc"}
$ 
$ curl -X GET http://127.0.0.1:8080/CFLBMHTMUZCG/task/1444a869-998
Not found task 1444a869-998
$
$ curl -X GET http://127.0.0.1:8080/CFLBMHTMUZCG/task
{"id":"d8593370-20c5-4fc7-a549-75748b876f7c","user_id":"CFLBMHTMUZCG","tasks":["cea30ba5-d68"],"create_at":"2022-10-12T20:53:43.583625422Z","time_zone":"utc"}
```
