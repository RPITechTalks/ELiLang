# Docker image for ELiLang.  Includes LLVM 10.0 and other necessary
# dependencies.  By default, this just spawns a REPL so you can run
# it without needing to specify any arguments/copy scripts into the
# container.
#
# Updating the compiler requires you to rebuild this container.

FROM alpine:3.12

RUN apk add --no-cache llvm10-dev rust cargo

WORKDIR /ELi

COPY . .

RUN llvm-config --ldflags

RUN cargo build --release

CMD ["cargo", "run", "--release", "--", "--repL"]
