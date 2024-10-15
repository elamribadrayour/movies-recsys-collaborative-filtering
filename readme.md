# Movies Recommendation Collaborative Filtering API

Welcome to the Movies Recommendation Collaborative Filtering API, a Rust-powered service that leverages collaborative filtering to provide item recommendations based on user ratings. This API is built using the Actix-web framework and utilizes a sparse matrix for efficient data handling.

## Features

- **Collaborative Filtering**: Uses user-item ratings to recommend items.
- **Efficient Sparse Matrix Operations**: Handles large datasets efficiently with `sprs`.
- **RESTful API**: Built with Actix-web for robust performance and scalability.
- **Asynchronous Processing**: Utilizes Tokio for handling asynchronous operations.

## Getting Started

These instructions will get your API up and running on your local machine for development and testing purposes.

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Cargo**: Rust's package manager, which comes with the Rust installation.

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/elamribadrayour/movies-recsys-collaborative-filtering.git
   cd movies-recsys-collaborative-filtering
   ```

2. **Install Dependencies**:

   Run the following command to fetch all dependencies:

   ```bash
   cargo build
   ```

### Running the Server

Start the server locally by running:

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`.

### API Endpoints

#### Health Check

- **GET** `/health`

  Check if the server is running.

  ```bash
  curl http://127.0.0.1:8080/health
  ```

#### Get Recommendations

- **POST** `/recommendations`

  Get a list of recommended item IDs for a user, sorted by predicted rating.

  **Request Body** (JSON):

  ```json
  {
    "user_id": 1
  }
  ```

  **Example**:

  ```bash
  curl -X POST http://127.0.0.1:8080/recommendations \
       -H "Content-Type: application/json" \
       -d '{"user_id": 1}'
  ```

### Architecture

- **Actix-web**: Provides the web server framework for handling HTTP requests.
- **sprs**: Utilized for efficient sparse matrix operations, allowing scalable data processing.
- **Tokio**: Powers asynchronous execution, ensuring high performance.

### Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements, bug fixes, or new features.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Contact

For any questions or suggestions, feel free to reach out to [badrayour.elamri@protonmail.com](mailto:badrayour.elamri@protonmail.com).
