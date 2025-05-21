FROM rust:1.79 as builder
WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    python3-venv \
    && apt-get clean

RUN pip3 install --no-cache-dir --break-system-packages \
    sentence-transformers \
    scikit-learn \
    numpy

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/backend"]