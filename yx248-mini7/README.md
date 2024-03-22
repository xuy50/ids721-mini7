# ids721-mini7

![pipeline status](https://gitlab.com/dukeaiml/IDS721/yx248-mini7/badges/main/pipeline.svg)

## Create a Cargo Function Microservice Project

1. Install Rust and Cargo
Rust is a programming language that Cargo, its package manager, uses. To install Rust and Cargo on Ubuntu, open your terminal and run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will download a script and start the installation. You'll need to follow the on-screen instructions.

2. Configure the Current Shell
After installation, configure your current shell by running:

```bash
source $HOME/.cargo/env
```

3. Create a New Cargo Project
To create a new Cargo project, use the cargo new command. For a microservice, you might want to start with a simple binary project:

```bash
cargo new yx248-mini7 --bin
```

This command creates a new directory named my_microservice with the basic structure of a Rust project.

4. Add Dependencies
Edit the `Cargo.toml` file to add necessary dependencies. For a microservice, you might use frameworks like Actix-web, Rocket, or Warp. Here's an example using Actix-web:

```toml
axum = "0.7.4"
tokio = { version = "1.36.0", features = ["macros", "full"] }
tower = "0.4.13"
csv = "1.1"
# json
serde_json = "1.0.113"
serde = { version = "1.0.196", features = ["derive"] }
# openssl
openssl = { version = "0.10", features = ["vendored"] }
# qdrant
reqwest = { version = "0.11", features = ["json"] }
uuid = { version = "1", features = ["serde", "v4"] }
```

Always check for the latest version of these libraries.


## Add Your Own Functions for Microservice

1. Create a new file named `lib.rs` to hold your data processing functions and tests.

2. Modify your `main.rs` to fit your function requirements and your router structure.

- For my function, it is based on my mini-project2 function:
[yx248-mini2 Link](https://gitlab.com/dukeaiml/IDS721/yx248-mini2)


## Build and Run the Project Local

Navigate to your project directory in the terminal and run the project with Cargo:

```bash
cargo run
```

Your microservice should now be running on localhost at the specified port (e.g., 127.0.0.1:3000).


## Local Testing
Test your microservice by navigating to the address in a web browser or using tools like `curl`:

```bash
curl http://127.0.0.1:8080/
```


## Build Docker Container

- To build and run the Docker container without environment variables (e.g., for testing or development environments), use the following commands:

```bash
# Build the Docker image
sudo docker build -t yx248-mini7-image .

# Run the container in detached mode (background) mapping port 3000
sudo docker run -dp 3000:3000 yx248-mini7-image

# List running containers to find the container ID
sudo docker ps

# Stop the running container using its ID
sudo docker stop <container_id>

# Remove the stopped container by its ID
sudo docker rm <container_id>
```

Replace <container_id> with the actual ID of your Docker container, which you can find using sudo docker ps.

- If you need to include sensitive information or environment-specific variables (like AWS credentials) in your build, it's recommended to use a `docker-compose.yml` file along with a `.env` file. This approach keeps sensitive data out of the Dockerfile and image itself:

```bash
# Build the Docker image using docker-compose
sudo docker-compose build

# Run the container in detached mode (background) using docker-compose
sudo docker-compose up -d

# List running containers to find the container ID
sudo docker ps

# To stop and remove the running container, use docker-compose
sudo docker-compose down
```

- Here are the screenshot for my docker build, up, ps, and down operations:
![Docker Build](images/docker-compose_build.png)
![Docker Up Ps Down](images/docker-compose_up_ps_down.png)

## Vector Database


## Vector Database Integration with Qdrant

This microservice leverages the Qdrant vector database for storing and querying vectorized data. Below are examples of how the microservice interacts with Qdrant, including creating collections, deleting collections, adding points to a collection, searching for points, and scrolling through points based on specific filters.

### Creating a Collection

To store vectors, a collection with a defined vector size and distance metric is required. Here's how a collection is created:

```json
PUT collections/my_collection
{
  "vectors": {
    "size": 2,
    "distance": "Cosine"
  }
}
```

### Deleting a Collection

To remove an existing collection and all its data:

```json
DELETE collections/my_collection
```

### Adding Points to a Collection

Points, along with their payload, can be added to a collection as follows:

```json
PUT collections/my_collection/points
{
  "points": [
    {
      "id": 1,
      "vector": [1.2, 50],
      "payload": {
        "date": "2023-09-01",
        "product": "Apple",
        "price": 1.2,
        "quantity": 50
      }
    }
  ]
}
```

### Searching for Points

To search for points in a collection based on their vector:

```json
POST collections/my_collection/points/search
{
  "vector": [1.2, 50],
  "limit": 3,
  "with_payload": true
}
```

### Scrolling Through Points with Filters

To scroll through points in a collection based on a range filter:

```json
POST collections/my_collection/points/scroll
{
  "filter": {
    "should": [
      {
        "key": "price",
        "range": {
          "gte": 1,
          "lte": 2
        }
      }
    ]
  }
}
```

### Screenshots

Below are screenshots showcasing the results obtained when running various operations of the microservice:

- Adding data via terminal and web interface:
![add_data_terminal_output](images/add_data_terminal_output.png)
![add_data_webpage_output](images/add_data_webpage_output.png)

- Filtering data via terminal and web interface:
![filter_terminal_output](images/filter_terminal_output.png)
![filter_webpage_output](images/filter_webpage_output.png)
