# Rust Skeleton
The following repo is just a skeleton used as _almost_ empty template to start any Rust project. It mainly 
solves the problem of downloading dependencies of a Rust binary time and time again when a docker image when
a binary is built.

## Dependencies issues and docker
Rust compiles statically all the dependencies of a project and that takes a lot of time. To avoid this problem,
dependencies are cached by Cargo but this approach does not work inside a docker container and dependencies are
compiled every time an image is built. To avoid this problem, a dummy empty binary is generated inside the 
Docker image that download the dependencies of the binaries/libs in the project. With this approach, if the user
modifies the code of the programs, the dependencies are not going to be downloaded and compiled again since are
cached by Docker.

### Example of the build time inside the docker-image
```
etorres@Helena:~/Development/rust-skeleton$ make docker-image
docker build --build-arg PROJECT_NAME=rust-skeleton -f Dockerfile -t "rust-skeleton:latest" .
Sending build context to Docker daemon  1.624GB
Step 1/11 : FROM rust:1.44.1
 ---> be36dd54f964
Step 2/11 : ARG PROJECT_NAME=rust-skeleton
 ---> Using cache
 ---> a8b6bd114963
Step 3/11 : RUN USER=root cargo new --bin ${PROJECT_NAME}
 ---> Using cache
 ---> a9037499547d
Step 4/11 : WORKDIR /${PROJECT_NAME}
 ---> Using cache
 ---> 0bf72a5520fd
Step 5/11 : COPY ./Cargo.lock ./Cargo.toml ./
 ---> Using cache
 ---> 4957d9bdf0bf
Step 6/11 : RUN cargo build --release && rm -rf src/*.rs
 ---> Using cache
 ---> 93e0f3024639
Step 7/11 : COPY ./src ./src
 ---> 2300697c980e
Step 8/11 : RUN cargo build --release
 ---> Running in 2ce8f2edb89c
   Compiling rust-skeleton v0.1.0 (/rust-skeleton)
    Finished release [optimized] target(s) in 4.12s
Removing intermediate container 2ce8f2edb89c
 ---> ad10d354a010
Step 9/11 : FROM deploy-base:latest
 ---> ef56c96e43a6
Step 10/11 : ARG PROJECT_NAME=rust-skeleton
 ---> Using cache
 ---> 9ceac96684f7
Step 11/11 : COPY --from=0 /${PROJECT_NAME}/target/release/producer     /$PROJECT_NAME/target/release/consumer /
 ---> 01fe18c6b087
Successfully built 01fe18c6b087
Successfully tagged rust-skeleton:latest
```

## Code in the project
The project has a code example a Rabbitmq producer-consumer example. As a consequence, both binaries uses the
dependency [amiquip](https://docs.rs/amiquip/0.3.3/amiquip/)