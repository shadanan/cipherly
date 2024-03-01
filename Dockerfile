FROM rust AS backend

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
RUN cargo build --release

FROM node AS frontend

WORKDIR /app

COPY frontend/*.json frontend/*.js frontend/*.cjs frontend/*.ts ./
RUN npm install

COPY frontend/src ./src
COPY frontend/static ./static
RUN npm run build

FROM gcr.io/distroless/cc-debian12 AS runtime

WORKDIR /app

COPY --from=backend /app/target/release/cipherly ./
COPY --from=frontend /app/build /app/static

ENV PORT=8000
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE ${PORT}

CMD ["./cipherly"]
