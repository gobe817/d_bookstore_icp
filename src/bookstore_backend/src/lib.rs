#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// UserRole Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserRole {
    #[default]
    Admin,
    StoreManager,
    Customer,
}

// Book Status Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum BookStatus {
    #[default]
    Available,
    Sold,
    Reserved,
}

// Book Genre Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum BookGenre {
    #[default]
    Fiction,
    NonFiction,
    Mystery,
    Science,
    Biography,
    Fantasy,
    Other,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Book {
    id: u64,
    title: String,
    description: String,
    status: BookStatus,
    genre: BookGenre,
    created_at: u64,
    created_by: u64,
    assigned_to: Option<u64>, // Customer ID who reserved the book
    history: Vec<BookHistory>,
    comments: Vec<Comment>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct BookHistory {
    status: String,
    changed_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Comment {
    customer_id: u64,
    content: String,
    commented_at: u64,
}

// BookAsset Type Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum AssetType {
    #[default]
    Hardcover,
    Paperback,
    Ebook,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct BookAsset {
    id: u64,
    asset_name: String,
    asset_type: AssetType,
    purchase_date: u64,
    assigned_to: u64,
    approx_value: f64,      // Approximate value of the asset
    depreciation_rate: f64, // Annual depreciation rate as a percentage
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Customer {
    id: u64,
    username: String,
    role: UserRole,
    created_at: u64,
}

impl Storable for Book {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 4096;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for BookAsset {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for BookAsset {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Customer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Customer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static BOOK_STORAGE: RefCell<StableBTreeMap<u64, Book, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static BOOK_ASSET_STORAGE: RefCell<StableBTreeMap<u64, BookAsset, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static CUSTOMER_STORAGE: RefCell<StableBTreeMap<u64, Customer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Book Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct BookPayload {
    title: String,
    description: String,
    genre: BookGenre,
}

// BookAssetPayload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct BookAssetPayload {
    asset_name: String,
    asset_type: AssetType,
    purchase_date: u64,
    assigned_to: u64,
    approx_value: f64,
    depreciation_rate: f64,
}

// CustomerPayload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct CustomerPayload {
    username: String,
    role: UserRole,
}

// calculate_depreciation Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct CalculateDepreciationPayload {
    book_asset_id: u64,
    years: u64,
}

// assign_book
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AssignBookPayload {
    book_id: u64,
    assigned_to: u64,
}

// add_book_comment
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AddBookCommentPayload {
    book_id: u64,
    customer_id: u64,
    content: String,
}

// update_book_status Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UpdateBookStatusPayload {
    id: u64,
    status: BookStatus,
}

// Message
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    UnAuthorized(String),
}

#[ic_cdk::update]
fn create_customer(payload: CustomerPayload) -> Result<Customer, Message> {
    if payload.username.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'username' and 'role' are provided.".to_string(),
        ));
    }

    // Check if the customer already exists
    let customer_exists = CUSTOMER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, customer)| customer.username == payload.username)
    });
    if customer_exists {
        return Err(Message::Error("Customer already exists".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let customer = Customer {
        id,
        username: payload.username,
        role: payload.role,
        created_at: current_time(),
    };
    CUSTOMER_STORAGE.with(|storage| storage.borrow_mut().insert(id, customer.clone()));
    Ok(customer)
}

#[ic_cdk::query]
fn get_customers() -> Result<Vec<Customer>, Message> {
    CUSTOMER_STORAGE.with(|storage| {
        let customers: Vec<Customer> = storage
            .borrow()
            .iter()
            .map(|(_, customer)| customer.clone())
            .collect();

        if customers.is_empty() {
            Err(Message::NotFound("No customers found".to_string()))
        } else {
            Ok(customers)
        }
    })
}

#[ic_cdk::query]
fn get_customer_by_id(id: u64) -> Result<Customer, Message> {
    CUSTOMER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, customer)| customer.id == id)
            .map(|(_, customer)| customer.clone())
            .ok_or(Message::NotFound("Customer not found".to_string()))
    })
}

// Customer authentication
fn authenticate_customer(payload: CustomerPayload) -> Result<Customer, Message> {
    CUSTOMER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, customer)| customer.username == payload.username && customer.role == payload.role)
            .map(|(_, customer)| customer.clone())
            .ok_or(Message::UnAuthorized("Invalid credentials".to_string()))
    })
}

// Function to create book
#[ic_cdk::update]
fn create_book(payload: BookPayload, customer_payload: CustomerPayload) -> Result<Book, Message> {
    // Authenticate the customer
    let customer = authenticate_customer(customer_payload)?;
    if customer.role != UserRole::Customer && customer.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to create a book".to_string(),
        ));
    }

    // Ensure 'title' and 'description' are provided
    if payload.title.is_empty() || payload.description.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'title' and 'description' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let book = Book {
        id,
        title: payload.title,
        description: payload.description,
        status: BookStatus::Available,
        genre: payload.genre,
        created_at: current_time(),
        created_by: customer.id,
        assigned_to: None,
        history: vec![],
        comments: vec![],
    };
    BOOK_STORAGE.with(|storage| storage.borrow_mut().insert(id, book.clone()));
    Ok(book)
}

// Function to assign a book
#[ic_cdk::update]
fn assign_book(
    payload: AssignBookPayload,
    customer_payload: CustomerPayload,
) -> Result<Book, Message> {
    // Authenticate the customer
    let customer = authenticate_customer(customer_payload)?;
    if customer.role != UserRole::StoreManager && customer.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to assign a book".to_string(),
        ));
    }

    // Validate the assigned customer
    let assigned_customer_exists = CUSTOMER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, customer)| customer.id == payload.assigned_to)
    });

    if !assigned_customer_exists {
        return Err(Message::InvalidPayload(
            "Assigned customer does not exist".to_string(),
        ));
    }

    // Check if the book exists
    let book_exists = BOOK_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, book)| book.id == payload.book_id)
    });

    if !book_exists {
        return Err(Message::NotFound("Book not found".to_string()));
    }

    BOOK_STORAGE.with(|storage| {
        let mut storage_ref = storage.borrow_mut();
        if let Some(book) = storage_ref.get(&payload.book_id).iter_mut().next() {
            book.assigned_to = Some(payload.assigned_to);
            storage_ref.insert(payload.book_id, book.clone());
            Ok(book.clone())
        } else {
            Err(Message::NotFound("Book not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn update_book_status(payload: UpdateBookStatusPayload) -> Result<Book, Message> {
    BOOK_STORAGE.with(|book_storage| {
        let mut storage_ref = book_storage.borrow_mut();

        // Check if the book exists
        if let Some(mut book) = storage_ref.get(&payload.id) {
            // Update the status
            book.status = payload.status;

            // Update the history
            book.history.push(BookHistory {
                status: format!("{:?}", payload.status),
                changed_at: current_time(),
            });

            // Update the book in storage
            storage_ref.insert(payload.id, book.clone());

            Ok(book.clone())
        } else {
            Err(Message::NotFound("Book not found".to_string()))
        }
    })
}

// Function to add_book_comment
#[ic_cdk::update]
fn add_book_comment(payload: AddBookCommentPayload) -> Result<Book, Message> {
    BOOK_STORAGE.with(|book_storage| {
        let mut storage_ref = book_storage.borrow_mut();

        // Check if the book exists
        if let Some(mut book) = storage_ref.get(&payload.book_id) {
            // Check if the customer exists
            let customer_exists = CUSTOMER_STORAGE.with(|customer_storage| {
                customer_storage
                    .borrow()
                    .iter()
                    .any(|(_, customer)| customer.id == payload.customer_id)
            });

            if !customer_exists {
                return Err(Message::InvalidPayload("Customer does not exist".to_string()));
            }

            // Add the comment
            book.comments.push(Comment {
                customer_id: payload.customer_id,
                content: payload.content,
                commented_at: current_time(),
            });

            // Update the book in storage
            storage_ref.insert(payload.book_id, book.clone());

            Ok(book.clone())
        } else {
            Err(Message::NotFound("Book not found".to_string()))
        }
    })
}

// Function to get Books
#[ic_cdk::query]
fn get_books() -> Result<Vec<Book>, Message> {
    BOOK_STORAGE.with(|storage| {
        let books: Vec<Book> = storage
            .borrow()
            .iter()
            .map(|(_, book)| book.clone())
            .collect();

        if books.is_empty() {
            Err(Message::NotFound("No books found".to_string()))
        } else {
            Ok(books)
        }
    })
}

#[ic_cdk::query]
fn get_book_by_id(id: u64) -> Result<Book, Message> {
    BOOK_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, book)| book.id == id)
            .map(|(_, book)| book.clone())
            .ok_or(Message::NotFound("Book not found".to_string()))
    })
}

// Function to create a book asset
#[ic_cdk::update]
fn create_book_asset(payload: BookAssetPayload, customer_payload: CustomerPayload) -> Result<BookAsset, Message> {
    // Authenticate the customer
    let customer = authenticate_customer(customer_payload)?;
    if customer.role != UserRole::StoreManager && customer.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to create a book asset.".to_string(),
        ));
    }

    // Ensure 'asset_name' and 'asset_type' are provided
    if payload.asset_name.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'asset_name' and 'asset_type' are provided.".to_string(),
        ));
    }

    // Validate the assigned customer
    let assigned_customer_exists = CUSTOMER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, customer)| customer.id == payload.assigned_to)
    });

    if !assigned_customer_exists {
        return Err(Message::InvalidPayload(
            "Assigned customer does not exist".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let book_asset = BookAsset {
        id,
        asset_name: payload.asset_name,
        asset_type: payload.asset_type,
        purchase_date: payload.purchase_date,
        assigned_to: payload.assigned_to,
        approx_value: payload.approx_value,
        depreciation_rate: payload.depreciation_rate,
    };
    BOOK_ASSET_STORAGE.with(|storage| storage.borrow_mut().insert(id, book_asset.clone()));
    Ok(book_asset)
}

#[ic_cdk::query]
fn get_book_assets() -> Result<Vec<BookAsset>, Message> {
    BOOK_ASSET_STORAGE.with(|storage| {
        let book_assets: Vec<BookAsset> = storage
            .borrow()
            .iter()
            .map(|(_, book_asset)| book_asset.clone())
            .collect();

        if book_assets.is_empty() {
            Err(Message::NotFound("No book assets found".to_string()))
        } else {
            Ok(book_assets)
        }
    })
}

#[ic_cdk::query]
fn get_book_asset_by_id(id: u64) -> Result<BookAsset, Message> {
    BOOK_ASSET_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, book_asset)| book_asset.id == id)
            .map(|(_, book_asset)| book_asset.clone())
            .ok_or(Message::NotFound("Book asset not found".to_string()))
    })
}

// Function to calculate depreciation
#[ic_cdk::update]
fn calculate_depreciation(payload: CalculateDepreciationPayload) -> Result<f64, Message> {
    BOOK_ASSET_STORAGE.with(|storage| {
        if let Some(book_asset) = storage.borrow().get(&payload.book_asset_id) {
            let years = payload.years as f64;
            let depreciation_rate = book_asset.depreciation_rate / 100.0;
            let depreciation = (1.0 - depreciation_rate).powf(years);
            let current_value = 1000.0; // Assuming the initial value of the book asset is $1000
            let value = current_value * depreciation;
            Ok(value)
        } else {
            Err(Message::NotFound("Book asset not found".to_string()))
        }
    })
}
// Helper function to get the current time
fn current_time() -> u64 {
    time()
}

// Export the candid functions
ic_cdk::export_candid!();
