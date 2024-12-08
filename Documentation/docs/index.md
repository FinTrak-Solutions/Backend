```text
 ______  __  __   __  ______  ______  ______  ______  __  __    
/\  ___\/\ \/\ "-.\ \/\__  _\/\  == \/\  __ \/\  ___\/\ \/ /    
\ \  __\\ \ \ \ \-.  \/_/\ \/\ \  __<\ \  __ \ \ \___\ \  _"-.  
 \ \_\   \ \_\ \_\\"\_\ \ \_\ \ \_\ \_\ \_\ \_\ \_____\ \_\ \_\ 
  \/_/    \/_/\/_/ \/_/  \/_/  \/_/ /_/\/_/\/_/\/_____/\/_/\/_/
```
---
# 🚀 **Backend Documentation**

This documentation outlines the backend structure, API endpoints, project layout, and the process for updating and deploying the documentation. The backend is built with **Rust Rocket** for REST API handling, **Diesel** for database interaction, and **PostgreSQL** for persistent storage.

---

## 📚 **Table of Contents**

1. [🌐 API](#-api)
    - [🔐 Authentication](#authentication)
    - [📘 Account Management](#account-management)
    - [📦 Category Management](#category-management)
    - [💸 Transaction Management](#transaction-management)
2. [📮 Postman API Testing](#-postman-api-testing)
3. [⚡ Quick Start](#-quick-start)
4. [🗄️ Database Schema](#-database-schema)
5. [🗂️ Project Layout](#-project-layout)
6. [🛠️ How to Add a New Module](#%EF%B8%8F-how-to-add-a-new-module)

---

## 🌐 **API**

### 🔐 **Authentication**
| **API**                                    | **Status**  | **Time Finished**  | **Link to Docs**                            |
|--------------------------------------------|-------------|---------------------|--------------------------------------------|
| `/signup`                                  | ✅ Complete | 2024-12-07 2:00pm   | [View Docs](authentication/#signup-post)   |

---

### 📘 **Account Management**
| **API**                                    | **Status**  | **Time Finished**  | **Link to Docs**                                      |
|--------------------------------------------|-------------|---------------------|-----------------------------------------------------|
| `/account_create`                          | ✅ Complete | 2024-12-07 3:10pm   | [View Docs](account/#create-new-account-for-user-post)|
| `/account_summary?email=<>`                | ✅ Complete | 2024-12-07 3:40pm   | [View Docs](account/#get-account-overview-for-user-get)|
| `/delete_account?email=<>&account_name=<>` | ✅ Complete | 2024-12-07 4:20pm   | [View Docs](account/#delete-an-account-for-user-delete)|
| `/account_detail`                          | ❌ In Progress | TBD               | [View Docs](account/#get-account-detailed-view-for-user-get) |

---

### 📦 **Category Management**
| **API**                                    | **Status**   | **Time Finished**  | **Link to Docs**                                        |
|--------------------------------------------|--------------|---------------------|-------------------------------------------------------|
| `/category_create`                         | ✅ Complete  | 2024-12-07          | [View Docs](category/#create-new-category-post)         |
| `/category_summary?email=<>`               | ✅ Complete  | 2024-12-07          | [View Docs](category/#get-category-overview-for-user-get)|
| `/delete_category?email=<>&category_nickname=<>` | ✅ Complete | 2024-12-07         | [View Docs](category/#delete-a-category-for-user-delete)|
| `/category_update?email=<>&field=<field_to_update>&category_nickname=<>&new_value=<>`| ✅ Complete | 2024-12-07         | [View Docs](category/#update-a-category-for-user-update) |

---

### 💸 **Transaction Management**
| **API**                                    | **Status**    | **Time Finished**  | **Link to Docs**                          |
|--------------------------------------------|---------------|---------------------|------------------------------------------|
| TBD                                       | 🚧 In Progress | TBD                 | N/A                                      |

---

## 📮 **Postman API Testing**
To explore and test the API endpoints, you can check out the Postman API documentation [here](https://web.postman.co/workspace/46a5447a-bfb7-47fa-8a8b-0da03a25416e/collection/40276125-9521e786-da55-44fd-9b33-98f4b67d293e) (localhost version).

---

## ⚡ **Quick Start**

### 🔥 **Clone the project**
```bash
git clone https://github.com/FinTrak-Solutions/Backend.git
```

---

### 📦 **Install Virtual Environment**
```bash
cd Backend
virtualenv venv
source venv/bin/activate
pip3 install mkdocs
```

---

### 📁 **Go to the Documentation Directory**
```bash
cd Documentation
```

---

### 📝 **Modify `.md` files in `docs/`**
```bash
# Check and update locally
mkdocs serve
# Modify .md files in the docs/ folder
```

---

### 🚀 **Build and Deploy**
```bash
# Build and deploy
mkdocs build
mkdocs gh-deploy
```

---

## 🗄️ **Database Schema**
📘 Full details about the **Database Schema** can be found [here](database.md).

---

## 🗂️ **Project Layout**

Here is the visualized structure of the `src` directory for the backend.

```
src
├── db.rs  -- 🗄️ Handles database setup and connections
├── handlers -- 🛠️ Controllers for handling business logic
│   ├── account_handler.rs  -- 📘 Handles account-related logic
│   ├── auth_handler.rs  -- 🔐 Handles user authentication logic
│   ├── category_handler.rs  -- 📦 Handles category-related logic
│   └── mod.rs  -- 📦 Module declaration file for handlers
├── main.rs  -- 🚀 The main entry point for the backend application
├── models -- 📦 Data models that map to the database schema
│   ├── account.rs  -- 📘 Account model
│   ├── category.rs  -- 📦 Category model
│   ├── mod.rs  -- 📦 Module declaration file for models
│   ├── transaction.rs  -- 💸 Transaction model
│   └── user.rs  -- 🔐 User model
├── routes -- 🌐 Defines the routes for the API endpoints
│   ├── account.rs  -- 📘 Account-related API routes
│   ├── auth.rs  -- 🔐 Authentication-related API routes
│   ├── category.rs  -- 📦 Category-related API routes
│   ├── mod.rs  -- 📦 Module declaration file for routes
│   └── transaction.rs  -- 💸 Transaction-related API routes
└── schema.rs  -- 📘 Automatically generated schema file for Diesel
```

---

## 🛠️ **How to Add a New Module**

Want to add a new module (like `transaction` or `budget`)? Follow these steps to ensure a consistent, clean structure.

### 1️⃣ **Create the New Module**
1. Create a new folder in `src/handlers/`, `src/models/`, and `src/routes/` for your new module.
```bash
   touch src/handlers/new_module_handler.rs
   touch src/models/new_module.rs
   touch src/routes/new_module.rs
```

2. Add the new module to the `mod.rs` files in each of these folders.

**src/handlers/mod.rs**
```rust
pub mod new_module_handler;
```

**src/models/mod.rs**
```rust
pub mod new_module;
```

**src/routes/mod.rs**
```rust
pub mod new_module;
```

---

### 2️⃣ **Define the Database Schema**
1. Add the table to the `schema.rs` file if it doesn't exist.
2. Run Diesel to generate the schema for the new table:
```bash
   diesel migration generate create_new_module
```

---

### 3️⃣ **Add Business Logic**
1. Add business logic to `new_module_handler.rs`.
2. Implement CRUD functions like `create`, `read`, `update`, `delete`.

---

### 4️⃣ **Register Routes**
1. Add routes for the new module in `routes/new_module.rs`.
2. Use `Rocket` to define `GET`, `POST`, `PUT`, and `DELETE` endpoints.

**Example Route in `src/routes/new_module.rs`:**
```rust
use rocket::{get, post};

#[get("/new_module")]
pub fn get_new_module() -> &'static str {
    "Get all new module items"
}

#[post("/new_module")]
pub fn create_new_module() -> &'static str {
    "Create a new module item"
}
```

---

### 5️⃣ **Update main.rs**
1. Add the new module's route to the `main.rs` file.
2. Register the routes with `Rocket`.

**src/main.rs**
```rust
#[macro_use] extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            routes::new_module::get_new_module,
            routes::new_module::create_new_module,
        ])
}
```
