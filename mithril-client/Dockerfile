###############################
# STEP 1: build a small image
###############################
FROM debian:11-slim

# Create appuser
RUN adduser --no-create-home --disabled-password appuser

# Precreate workdir
RUN mkdir -p /app/bin

# Upgrade
RUN apt-get update -y && apt-get install -y libssl-dev ca-certificates wget sqlite3 && rm -rf /var/lib/apt/lists/*

# Install mithril-client
ARG MITHRIL_BIN_URL=https://github.com/input-output-hk/mithril/releases/download/2337.0/mithril-2337.0-linux-x64.tar.gz
RUN wget -nv -O mithril-2337.0-linux-x64.tar.gz $MITHRIL_BIN_URL \
    && tar xzf mithril-2337.0-linux-x64.tar.gz mithril-client && mv mithril-client /app/bin && chmod +x /app/bin/mithril-client \
    && /app/bin/mithril-client --version \
    && rm -f mithril-2337.0-linux-x64.tar.gz

# Copy the config files
COPY config /app/config

#Workdir
WORKDIR /app/
RUN chown -R appuser /app/

# Use an unprivileged user
USER appuser

# Run the executable
ENTRYPOINT ["/app/bin/mithril-client", "-vvv"]
