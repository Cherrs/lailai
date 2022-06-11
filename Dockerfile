FROM alpine
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY  /target/x86_64-unknown-linux-musl/release/lailai .

CMD ["./lailai"]