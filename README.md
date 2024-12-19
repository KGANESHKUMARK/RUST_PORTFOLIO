# My Rust Project

This is a simple Rust project that demonstrates the basic structure of a Rust application.

## Getting Started

To build and run this project, you need to have Rust and Cargo installed on your machine. You can install them by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

## Building the Project

To build the project, navigate to the project directory and run the following command:

```
cargo build
```

## Running the Application

After building the project, you can run the application using:

```
cargo run
```

## Project Structure

- `src/main.rs`: The entry point of the application.
- `Cargo.toml`: The configuration file for the Cargo package manager.
- `README.md`: Documentation for the project.

## Code coverage
- to install : cargo install cargo-tarpaulin
- cmd : 
```
cargo tarpaulin --out html

cargo tarpaulin --out Xml
```

## # Build the project
```
cargo build --release
```


## run test
```
cargo test
```
## Compilateion mechanism
Cargo, the Rust package manager and build system, handles the compilation and execution of Rust projects. It uses a combination of metadata, dependency management, and build scripts to determine whether it needs to compile the code or if it can simply run the existing binaries. Here's an overview of how Cargo's compilation mechanism works:

How Cargo Determines Whether to Compile
Dependency Graph: Cargo constructs a dependency graph based on the Cargo.toml file. This graph includes all the dependencies and their versions.

Timestamps: Cargo checks the timestamps of the source files and the compiled artifacts. If any source file has been modified since the last compilation, Cargo will recompile the affected parts of the project.

Cargo.lock: Cargo uses the Cargo.lock file to ensure that the exact versions of dependencies are used. If the Cargo.lock file changes (e.g., due to adding or updating dependencies), Cargo will recompile the project.

Build Scripts: If the project or any of its dependencies include build scripts (build.rs), Cargo will run these scripts to determine if any additional compilation steps are needed.

<!-- Compilation Mechanism -->
 <!-- Parsing and Analysis:  -->
Cargo invokes the Rust compiler (rustc) to parse and analyze the source code. This step includes syntax checking, type checking, and borrow checking.

<!-- Intermediate Representation (IR):  -->
The Rust compiler generates an intermediate representation (IR) of the code, which is used for further optimization and code generation.

<!-- Code Generation:  -->
The Rust compiler generates machine code from the IR. This step includes optimizations based on the selected profile (e.g., dev or release).

<!-- Linking:  -->
The generated machine code is linked with the necessary libraries and dependencies to produce the final executable binary.

<!-- Cargo Commands -->
Build: The cargo build command compiles the project. It checks the timestamps and dependency graph to determine if recompilation is needed.

<!-- cargo build -->

Run: The cargo run command compiles the project (if needed) and then runs the resulting binary.

<!-- cargo run -->

Test: The cargo test command compiles the project (if needed) and runs the tests.

<!-- cargo test -->

Clean: The cargo clean command removes the target directory, forcing a full recompilation on the next build.

<!-- cargo clean -->


PostgreSQL:

Why: PostgreSQL is a powerful, open-source relational database known for its advanced features, performance, and scalability. It has strong support in the Rust ecosystem with libraries like sqlx and diesel.
Libraries: sqlx, diesel
Use Cases: Web applications, data warehousing, and complex queries.
MySQL/MariaDB:

Why: MySQL is a widely-used open-source relational database known for its reliability and performance. MariaDB is a fork of MySQL with additional features and improvements.
Libraries: sqlx, diesel
Use Cases: Web applications, e-commerce, and content management systems.
SQLite:

Why: SQLite is a lightweight, serverless, self-contained SQL database engine. It is suitable for smaller applications, prototyping, and applications where simplicity and ease of setup are important.
Libraries: sqlx, rusqlite
Use Cases: Embedded systems, mobile applications, and small to medium-sized applications.
MongoDB:

Why: MongoDB is a popular NoSQL database known for its flexibility, scalability, and ease of use. It is suitable for applications that require a flexible schema and high scalability.
Libraries: mongodb
Use Cases: Real-time analytics, content management, and IoT applications.
Redis:

Why: Redis is an in-memory data structure store used as a database, cache, and message broker. It is known for its high performance and support for various data structures.
Libraries: redis
Use Cases: Caching, real-time analytics, and session management.

sqlx = { version = "0.5", features = ["runtime-tokio-rustls", "postgres", "mysql", "sqlite"] }
--oracle = "0.5"
--odbc = "0.17"
--dotenv = "0.15"

--code coverage
Sonarqube currently does't have rust support. You will also need Sonar-Rust, a rust plugin for sonarqube to read the results generated in clippy report file. Login to sonarqube and view your resutls after. Scan Rust Code With Sonarqube. GitHub Gist: instantly share code, notes, and snippets.


Creating a front-end UI using Rust can be done using frameworks like Yew or Seed, which allow you to build web applications with Rust. Here, I'll provide a simple example using the Yew framework.

Step 1: Set Up Your Project
First, create a new Rust project for the front end:

Add the necessary dependencies to your Cargo.toml:

Cargo.toml
Step 2: Create the Front-End Application
Create a new file src/lib.rs and add the following code:

src/lib.rs
Step 3: Build the Front-End Application
To build the front-end application, you need to install wasm-pack:

Then, build the project using wasm-pack:

Step 4: Serve the Front-End Application
Create an index.html file to serve the front-end application:

index.html
Serve the index.html file using a simple HTTP server. You can use basic-http-server for this:

Open your browser and navigate to http://localhost:4000 to see the front-end application.

This example demonstrates a simple front-end application using Yew. You can expand this example to interact with your Actix-web backend by making HTTP requests to the API endpoints.

## UI
cargo new rust_frontend --bin
cd rust_frontend


cargo install wasm-pack

wasm-pack build --target web

cargo install basic-http-server
basic-http-server .

## pluin to debug 

Install the Rust Extension: Install the Rust extension for Visual Studio Code.
Install the CodeLLDB Extension: Install the CodeLLDB extension for debugging support.
Configure Launch Settings: Create a launch.json file in the .vscode directory with the following content:



Step 1: Ensure CMake is Installed
Make sure cmake is installed and added to your system PATH. You can verify this by running:

Step 2: Install Required Build Tools
Ensure you have the necessary build tools installed. On Windows, you can install the Visual Studio Build Tools:

Go to the Visual Studio Build Tools download page.
Download and install the Build Tools for Visual Studio.
During installation, select the "Desktop development with C++" workload.
Step 3: Install OpenSSL
The rdkafka crate may require OpenSSL. You can install OpenSSL using vcpkg:

Clone the vcpkg repository:
Bootstrap vcpkg:
Install OpenSSL:
Integrate vcpkg with your project:
Step 4: Set Environment Variables
Set the environment variables to point to the OpenSSL installation:

Step 5: Build the Project
After ensuring all dependencies are installed and environment variables are set, try building your project again:

git clone https://github.com/microsoft/vcpkg.git
cd vcpkg

.\vcpkg install openssl:x64-windows

.\vcpkg install openssl:x64-windows

.\vcpkg integrate install

set OPENSSL_DIR=C:\path\to\vcpkg\installed\x64-windows
set OPENSSL_INCLUDE_DIR=%OPENSSL_DIR%\include
set OPENSSL_LIB_DIR=%OPENSSL_DIR%\lib