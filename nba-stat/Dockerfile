# FROM rust:latest as builder
# ENV APP nba-stat
# WORKDIR /usr/src/$APP
# COPY . .
# RUN cargo install --path .

# # Install Python and required packages
# RUN apt-get update && apt-get install -y python3 python3-pip
# RUN pip3 install nba_api

FROM rust:latest as builder
# Install Python 3.9 and development files
RUN apt-get update && apt-get install -y python3.9 python3.9-dev libpython3.9
RUN apt-get update && apt-get install -y python3 python3-pip

# Install glibc-utils to get the ldd command
RUN apt-get update && apt-get install -y libc-bin

RUN pip3 install nba_api
ENV LD_LIBRARY_PATH=/usr/local/lib
ENV APP nba-stat
# Set the WORKDIR to /usr/src/app
WORKDIR /usr/src/$APP
# Copy the Rust code into the container
COPY . .
RUN cargo install --path .
# Build the Rust code
# RUN cargo build
# # Run the Rust code
# CMD ["cargo", "run"]

 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["nba-stat"]