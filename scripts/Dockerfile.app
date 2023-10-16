FROM rust:1-alpine3.18

WORKDIR /app
RUN apk add libc-dev openssh-client git

COPY . .

RUN cargo fetch

RUN mkdir /root/.ssh
COPY scripts/.ssh/id_rsa /root/.ssh
COPY scripts/bootstrap.sh /root/bootstrap.sh
RUN chmod +x /root/bootstrap.sh


# Run with one thread, because the integration tests changes current folder
CMD ["ssh-agent", "/bin/sh", "-c", "sh /root/bootstrap.sh && cargo test -- --test-threads=1 --nocapture"]
