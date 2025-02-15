# SurrealDB Detective Board in Rust

This project demonstrates how to build a simple detective board application using SurrealDB and Rust. It covers creating a schema, connecting to the database, inserting data, and handling relationships between different entities (people, locations, organizations, events, and evidence). It focuses on correctly representing SurrealDB's data types, particularly record links and geometries, within Rust structs using `serde` for serialization and deserialization.

## Project Structure

- `src/main.rs`: Contains the Rust code for interacting with SurrealDB.
- `Cargo.toml`: Defines the project's dependencies.
- `README.md`: This file.

## Prerequisites

- **Rust:** Install Rust using `rustup` (https://www.rust-lang.org/tools/install).
- **SurrealDB:** Download and install SurrealDB from the official website (https://surrealdb.com/install).
- **SurrealDB Server Running:** You must have a SurrealDB server instance running locally. You can usually start it with: `surreal start --log debug --bind 0.0.0.0:8000 memory` or `surreal start --log debug --bind 0.0.0.0:8000 file:path/to/your/data.db`

## Dependencies

The project uses the following Rust crates (libraries):

- `surrealdb`: The SurrealDB client library for Rust.
- `tokio`: An asynchronous runtime for Rust.
- `serde`: A framework for serializing and deserializing Rust data structures.
- `serde_json`: For working with JSON data.

These dependencies are listed in the `Cargo.toml` file:

```toml
[dependencies]
chrono = "0.4.39"
serde = "1.0.217"
serde_json = "1.0.138"
surrealdb = "2.2.0"
tokio = { version = "1.43.0", features = ["full"] }
```

## Schema

The database schema defines the structure of the data. It includes tables for:

- `person`: Represents individuals.
- `location`: Represents places, including geospatial coordinates.
- `organization`: Represents groups or companies.
- `event`: Represents occurrences at a specific time.
- `evidence`: Represents items found or related to the investigation.

The schema also defines various relationships (edges) between these tables:

- `related_to`: A generic relationship between any two entities.
- `at`: Connects a person or event to a location.
- `member_of`: Connects a person to an organization.
- `involved_in`: Connects a person to an event.
- `found_at`: Connects evidence to a location.
- `related_evidence`: Connects evidence to a person, event, or organization.
- `linked_to`: Connects people to people or organizations to organizations.

The SurrealQL schema definition is provided in the problem description and is assumed to be already applied to your SurrealDB instance. You can apply it using the SurrealDB CLI:

```bash
surreal import --conn ws://localhost:8000/ --ns test --db detective schema.surql
```

(Replace `schema.surql` with the actual filename containing your schema definition).

## Code Explanation

The `src/main.rs` file contains the following key parts:

1.  **Connecting to SurrealDB:** The code connects to the SurrealDB server running at `ws://localhost:8000`, signs in with the username "root" and password "root", and selects the "test" namespace and "detective" database. _You should change these credentials and database names as needed._

2.  **Data Structures (Structs):** Rust structs are defined to represent the tables in the schema. These structs use `serde`'s `Serialize` and `Deserialize` derives for easy conversion to/from JSON, which is how SurrealDB communicates.

    - **`Geometry` Handling:** The `surrealdb::sql::Geometry` enum is used to represent geospatial data (points, lines, polygons, etc.). This ensures that you are using SurrealDB's native geometry types.
    - **`RecordId`:** Record ids (links) are handled using a wrapper type `RecordId` around the `surrealdb::sql::Id`.
    - **`GeometryFeature` struct**: Provides examples for all of surrealdb's geometry types, and creating features and adding properties to them.
    - **`Option<...>`:** Fields that are optional in the schema are represented as `Option<...>` in the Rust structs.
    - **Record Links with Different Tables (`RecordLink`):** For fields that can link to multiple tables (e.g., `related_to.out`), an enum `RecordLink` is used. This enum has a variant for each possible table, each holding a `RecordId`. The `#[serde(untagged)]` attribute is crucial for deserialization.

3.  **Creating Data:** The code provides examples of creating instances of these structs and inserting them into the database using `db.create().content(&data).await?`.

4.  **Error Handling:** The code uses `Result<(), SurrealError>` and the `?` operator for error handling. This ensures that any errors from SurrealDB are properly handled and propagated.

## Running the Code

1.  **Start SurrealDB:** Make sure your SurrealDB server is running (see Prerequisites).

2.  **Apply the Schema:** Apply the SurrealQL schema to your database instance (see Schema section).

3.  **Build and Run:** Use the following command in your terminal (from the project directory) to build and run the code:

    ```bash
    cargo run
    ```

The code will connect to the database, insert some example data, and print the results to the console. You can then use the SurrealDB web UI (usually at `http://localhost:8000`) to view the data.

## Further Development

This project provides a basic foundation. You can extend it by:

- **Adding Queries:** Implement functions to query the database based on various criteria (e.g., find all people at a specific location, find evidence related to a person).
- **Creating a User Interface:** Develop a user interface (web, desktop, or command-line) to interact with the data more easily.
- **Adding More Complex Logic:** Implement more sophisticated detective board features, such as tracking leads, suspects, and timelines.
- **Using a Persistent Database:** Change the SurrealDB startup command to use a file-based database instead of an in-memory database to persist data between sessions (`surreal start --log debug --bind 0.0.0.0:8000 file:path/to/your/database.db`).
- **Adding Authentication and Authorization:** Implement proper user authentication and authorization to secure access to the data.
- **Using SurrealDB Cloud**: Instead of running surrealdb locally, you may want to sign up for a cloud option.
