FROM rust:1.62.1
ARG PROJECT_NAME=rust-skeleton

# Do not use or modify. A dummy main program (src/main.rs) is added and built
# to load the dependencies of the project and cache them.
# Binaries/libs of the project can be added using this guide: https://doc.rust-lang.org/cargo/guide/project-layout.html 
# With this structure, changes to the code of the project won't 
# download all the dependencies again since they are cached by Docker.
# They are only going to be downloaded once or the next time an update is found 
RUN USER=root cargo new --bin ${PROJECT_NAME}
WORKDIR /${PROJECT_NAME}
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release && rm -rf src/*.rs
COPY ./src ./src
# RUN rm ./target/release/deps/${PROJECT_NAME}*
RUN cargo build --release

FROM deploy-base:latest
ARG PROJECT_NAME=rust-skeleton
COPY --from=0 /${PROJECT_NAME}/target/release/webserver /
COPY --from=0 /${PROJECT_NAME}/target/release/minigrep /