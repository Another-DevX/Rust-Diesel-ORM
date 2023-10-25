# Rust CRUD Application with Actix, Diesel, and PostgreSQL

## Overview
This repository contains a CRUD (Create, Read, Update, Delete) application built with Rust, using Actix as the web framework, Diesel as the ORM, and PostgreSQL as the database. The application also uses Tera for HTML template handling.

## Table of Contents

- [Rust CRUD Application with Actix, Diesel, and PostgreSQL](#rust-crud-application-with-actix-diesel-and-postgresql)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Diesel CLI](https://diesel.rs/guides/getting-started)

## Installation

1. **Clone the Repository**

    ```bash
    git clone https://github.com/Another-DevX/Rust-ORM-Backend.git
    cd Rust-ORM-Backend
    ```

2. **Set Up the Database**

    - Start PostgreSQL and create a new database.
    - Alternatively, you can run PostgreSQL through Docker:

        ```bash
        docker-compose up -d postgresql
        ```
        
    - Rename `.env.example` to `.env` and update it with your database information.
  
3. **Install Diesel CLI**

    - Make sure you have PostgreSQL installed.
  
    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```

4. **Run Migration**

    ```bash
    diesel migration run
    ```

5. **Build the Project**

    ```bash
    cargo build
    ```

## Usage

Run the project:

```bash
cargo run
