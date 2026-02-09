docker build -t axum_web_server:latest .
docker run --rm -d --name axum_web_server_app -p 3000:3000 axum_web_server:latest