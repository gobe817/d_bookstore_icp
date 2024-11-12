# Bookstore Canister

## Overview

The **Bookstore Canister** is a decentralized application (DApp) running on the **Internet Computer**. It manages a virtual bookstore with functionalities for customers, books, and book assets. This backend allows for user authentication, book creation and assignment, asset depreciation, comments on books, and more. 

This project utilizes **Candid** for type serialization, **StableBTreeMap** for persistent storage, and the **IC SDK** to implement the canister's functionality.

## Key Features

- **User Roles**: Three types of roles: Admin, Store Manager, and Customer.
- **Books**: Create books, assign them to customers, and update their status (Available, Sold, Reserved).
- **Book Assets**: Track assets associated with books (e.g., hardcover, paperback), calculate depreciation, and manage asset assignments.
- **Customer Management**: Create customers, authenticate them, and manage their roles.
- **Comments**: Customers can leave comments on books.
- **Persistence**: Stable storage is used to keep track of books, customers, and assets.

## Prerequisites

- **Rust**: Install the Rust programming language via [rust-lang.org](https://www.rust-lang.org/tools/install).
- **DFINITY SDK**: Install the DFINITY SDK to interact with the Internet Computer. You can follow the installation instructions [here](https://internetcomputer.org/docs/current/developers-guide/install/).
- **Cargo**: Ensure you have `cargo` (Rust's package manager) installed to manage dependencies and build the project.

## Installation

1. Clone the repository:
   ```bash
   git clone <repo-url>
   cd bookstore
   ```

2. Install dependencies:
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```

3. Install `candid-extractor` for generating the `.did` file:
   ```bash
   cargo install candid-extractor
   ```

4. Extract the Candid interface description:
   ```bash
   candid-extractor target/wasm32-unknown-unknown/release/bookstore.wasm > bookstore.did
   ```

5. Deploy the canister to the Internet Computer.

## Usage

### User Roles

- **Admin**: Full access to create customers, books, assets, and assign them.
- **Store Manager**: Can assign books and create book assets.
- **Customer**: Can view books, add comments, and reserve books.

### APIs

The canister exposes a variety of functions to interact with books, customers, and assets:

#### **Customer Management**

1. **Create Customer**
   - Endpoint: `create_customer`
   - Payload: `CustomerPayload`
   - Role required: Any user can create a customer.
   
2. **Get Customers**
   - Endpoint: `get_customers`
   - Fetches all customers.

3. **Get Customer by ID**
   - Endpoint: `get_customer_by_id`
   - Payload: `id: u64`
   - Returns the customer data for the specified ID.

#### **Book Management**

1. **Create Book**
   - Endpoint: `create_book`
   - Payload: `BookPayload`, `CustomerPayload` (auth)
   - Role required: Admin or Customer.
   - Creates a new book in the system.

2. **Get Books**
   - Endpoint: `get_books`
   - Fetches all books.

3. **Get Book by ID**
   - Endpoint: `get_book_by_id`
   - Payload: `id: u64`
   - Fetches a book by its ID.

4. **Update Book Status**
   - Endpoint: `update_book_status`
   - Payload: `UpdateBookStatusPayload`
   - Role required: Admin.
   - Updates the book's status (Available, Sold, Reserved).

5. **Assign Book**
   - Endpoint: `assign_book`
   - Payload: `AssignBookPayload`, `CustomerPayload` (auth)
   - Role required: Store Manager or Admin.
   - Assigns a book to a customer.

6. **Add Book Comment**
   - Endpoint: `add_book_comment`
   - Payload: `AddBookCommentPayload`
   - Role required: Customer.
   - Adds a comment to a book.

#### **Book Asset Management**

1. **Create Book Asset**
   - Endpoint: `create_book_asset`
   - Payload: `BookAssetPayload`, `CustomerPayload` (auth)
   - Role required: Store Manager or Admin.
   - Creates a book asset (e.g., hardcover, paperback).

2. **Get Book Assets**
   - Endpoint: `get_book_assets`
   - Fetches all book assets.

3. **Get Book Asset by ID**
   - Endpoint: `get_book_asset_by_id`
   - Payload: `id: u64`
   - Fetches a book asset by its ID.

4. **Calculate Depreciation**
   - Endpoint: `calculate_depreciation`
   - Payload: `CalculateDepreciationPayload`
   - Calculates the depreciation of a book asset.

### Authentication

The `CustomerPayload` must be provided in relevant endpoints that require authentication. The canister checks the customer’s role to ensure authorization before proceeding with operations.

---

Let me know if you need any further modifications!
# d_bookstore_icp
