FROM rust AS backend
# For ARM64, replace all:
#   `x86_64-unknown-linux-musl` -> `aarch64-unknown-linux-musl`

WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN mv target/x86_64-unknown-linux-musl/release/cipherly .

FROM node AS frontend

WORKDIR /app

COPY frontend/*.json frontend/*.js frontend/*.cjs frontend/*.ts ./
RUN npm install

COPY frontend/src ./src
COPY frontend/static ./static
RUN npm run build

FROM scratch AS runtime

WORKDIR /app

COPY --from=backend /app/cipherly ./
COPY --from=frontend /app/build ./static

ENV PORT=8000
ENV ROCKET_ADDRESS=0.0.0.0

CMD ["./cipherly"]
