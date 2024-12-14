# Use a Golang image as the base image for building the application
FROM golang:1.23-alpine as builder

LABEL DEVELOPER="ryanserfanru@gmail.com"
LABEL APPS-NAME="user-service"

# Set the current working directory in the container
WORKDIR /app

# Copy go.mod and go.sum and download dependencies
COPY user-service/go.mod ./
COPY user-service/go.sum ./

RUN go mod download

# Copy the entire project to the container
COPY user-service/. .

# Build the Go application
RUN go build -o user-service .

# Use a smaller base image to run the app
FROM alpine:3.18

# Install ca-certificates to make the app work correctly with HTTPS
RUN apk --no-cache add ca-certificates

# Set the working directory for the container
WORKDIR /app

# Copy the Go binary from the builder stage to the final image
COPY --from=builder /app/user-service .

# Expose the port the app will run on
EXPOSE 8080

# Command to run the app
CMD ["./user-service"]
