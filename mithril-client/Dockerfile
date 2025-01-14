###############################
# STEP 1: build rust executable
###############################
FROM rust:bullseye AS rustbuilder

# Create appuser
RUN adduser --no-create-home --disabled-password appuser

# Set workdir
WORKDIR /app

# Copy local dependencies
COPY . .

# Build the app using a dummy main in order to cache dependencies
RUN mv /app/mithril-client /app/mithril-client.1 && mkdir -p /app/mithril-client/src
COPY mithril-client/Cargo.toml /app/mithril-client/
RUN echo "fn  main () {}" > /app/mithril-client/src/main.rs
RUN cargo build --release -p mithril-client --manifest-path /app/mithril-client/Cargo.toml

# Rollback the rest of the files into the container
RUN rm -rf /app/mithril-client && mv /app/mithril-client.1 /app/mithril-client
COPY ./mithril-client/src/main.rs /app/mithril-client/src/

# Build the binary
RUN cargo build --release -p mithril-client
RUN /app/target/release/mithril-client --version

###############################
# STEP 2: build a small image
###############################
FROM debian:11-slim

# Upgrade
RUN apt-get update -y && apt-get install -y libssl-dev ca-certificates wget sqlite3 && rm -rf /var/lib/apt/lists/*

# Import the user and group files from the builder
COPY --from=rustbuilder /etc/passwd /etc/passwd

# Copy the executable
COPY --from=rustbuilder /app/target/release/mithril-client /app/bin/mithril-client

# Copy the config files
COPY --from=rustbuilder /app/mithril-client/config /app/config

#Workdir
WORKDIR /app/
RUN chown -R appuser /app/

# Use an unprivileged user
USER appuser

# Run the executable
ENTRYPOINT ["/app/bin/mithril-client", "-vvv"]
