docker build . -t ping-scanner:0.1.0

docker run -v ./config:/app/config -v ./out:/app/out --net=host ping-scanner:0.1.0
docker run -it -v config:/app/config -v out/:/app/out --entrypoint /bin/sh --net=host ping-scanner:0.1.0

docker run -it -v F:\admin\Rust\ping-scanner\config:/app/config -v F:\admin\Rust\ping-scanner\out:/app/out --net host --entrypoint /app/ping-scanner ping-scanner:0.1.0