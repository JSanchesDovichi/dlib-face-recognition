FROM lukemathwalker/cargo-chef:0.1.73-rust-1.92-slim-trixie AS chef

RUN apt-get update

RUN apt-get install -y \
  cmake \
  libdlib-dev \
  libopenblas-dev \
  mold \
  clang

COPY container_link_config.toml .cargo/config.toml

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin dfr_starter_template --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN mold -run cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN mold -run cargo build --release --bin dfr_starter_template

FROM alpine:3.23.2 AS unzipper
WORKDIR /downloaded_files
RUN apk add bzip2 wget curl

RUN wget -P /opt/ https://github.com/davisking/dlib-models/raw/master/dlib_face_recognition_resnet_model_v1.dat.bz2 \
    https://github.com/davisking/dlib-models/raw/master/mmod_human_face_detector.dat.bz2 \
    https://github.com/davisking/dlib-models/raw/master/shape_predictor_68_face_landmarks_GTX.dat.bz2

RUN bzip2 -d /opt/dlib_face_recognition_resnet_model_v1.dat.bz2 \
    /opt/mmod_human_face_detector.dat.bz2 \
    /opt/shape_predictor_68_face_landmarks_GTX.dat.bz2

# We do not need the Rust toolchain to run the binary!
FROM debian:trixie
RUN apt-get update

RUN apt-get install -y \
  libdlib19.2 \
  libopenblas0

WORKDIR /app
COPY --from=builder /app/target/release/dfr_starter_template /usr/local/bin
COPY --from=unzipper /opt/ /app/models/
COPY ./assets /app/assets
ENTRYPOINT ["/usr/local/bin/dfr_starter_template"]