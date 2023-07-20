# Use a Rust image as the base image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the project files into the Docker image
COPY . .

# Build the project inside the Docker container
RUN cargo build --release

# Expose the necessary ports for your application to run
EXPOSE 8000

# Start your application inside the Docker container
CMD ["cargo", "run", "--release"]