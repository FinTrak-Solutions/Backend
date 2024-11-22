# Database Schema Documentation

## Introduction

This document outlines the updated database schema for the financial tracker 
application backend, utilizing Rust Rocket for REST API handling, Diesel for database interaction, 
and PostgreSQL as the database. The schema is designed to efficiently store and manage `user information`, 
`financial accounts`, `transactions`, `budgets`, and `categories`, ensuring data scalability.

---

## Table of Contents

1. [User Table](#user-table)
2. [Account Table](#account-table)
3. [Transaction Table](#transaction-table)
4. [Budget Table](#budget-table)
5. [Category Table](#category-table)
6. [Summary of Updates](#summary-of-updates)

---

## User Table

### Description

Stores user credentials and personal information. Each user is uniquely identified and can have multiple accounts and budgets associated with them.

### Schema

| Field Name  | Data Type         | Constraints                  | Description                              |
|-------------|-------------------|------------------------------|------------------------------------------|
| `id`        | `SERIAL`          | Primary Key                  | Unique identifier for each user.         |
| `email`     | `VARCHAR(255)`    | Unique, Not Null             | User's email address.                    |
| `password`  | `VARCHAR(255)`    | Not Null                     | Hashed password for security.            |
| `first_name`| `VARCHAR(50)`     | Optional                     | User's first name.                       |
| `last_name` | `VARCHAR(50)`     | Optional                     | User's last name.                        |
| `created_at`| `TIMESTAMP`       | Default `NOW()`              | Account creation timestamp.              |
| `updated_at`| `TIMESTAMP`       | Default `NOW()`, On Update   | Last account update timestamp.           |

### Notes

- **Password Security**: Passwords must be stored securely using a hashing algorithm like bcrypt.
- **Email Uniqueness**: Enforce a unique constraint to prevent duplicate accounts.

---

## Account Table

### Description

Represents financial accounts linked to users, such as bank accounts or credit cards. Each account belongs to a user and holds financial transactions.

### Schema

| Field Name      | Data Type           | Constraints                       | Description                              |
|-----------------|---------------------|-----------------------------------|------------------------------------------|
| `account_id`    | `SERIAL`            | Primary Key                       | Unique identifier for each account.      |
| `user_id`       | `INTEGER`           | Foreign Key (`User.id`), Not Null | Identifier of the account owner.     |
| `bank_name`     | `VARCHAR(100)`      | Optional                          | Name of the financial institution.       |
| `account_number`| `VARCHAR(50)`       | Optional, Unique                  | Account number from the bank.            |
| `account_type`  | `ACCOUNT_TYPE_ENUM` | Not Null                          | Type of account (`Credit`, `Debit`, `Saving`). |
| `currency`      | `VARCHAR(10)`       | Default `'CAD'`                   | Currency type of the account balance.    |
| `created_at`    | `TIMESTAMP`         | Default `NOW()`                   | Account creation timestamp.              |
| `updated_at`    | `TIMESTAMP`         | Default `NOW()`, On Update        | Last account update timestamp.           |

### Enumerations

- **ACCOUNT_TYPE_ENUM**: An enumeration with values `'Credit'`, `'Debit'`, `'Saving'`.

### Notes

- **Foreign Key Constraint**: `user_id` references `User.id` to establish ownership.
- **Account Balance Calculation**: It's recommended to calculate account balance dynamically from transactions to maintain consistency.

---

## Transaction Table

### Description

Logs all financial transactions associated with accounts. Each transaction records details like amount, timestamp, and category.

### Schema

| Field Name      | Data Type           | Constraints                  | Description                              |
|-----------------|---------------------|------------------------------|------------------------------------------|
| `transaction_id`| `SERIAL`            | Primary Key                  | Unique identifier for each transaction.  |
| `account_id`    | `INTEGER`           | Foreign Key (`Account.account_id`), Not Null | Associated account.   |
| `amount`        | `DECIMAL(15, 2)`    | Not Null                     | Transaction amount (positive or negative). |
| `created_at`    | `TIMESTAMP`         | Default `NOW()`              | Transaction timestamp.                   |
| `description`   | `TEXT`              | Optional                     | Additional details about the transaction.|
| `category_id`   | `INTEGER`           | Foreign Key (`Category.category_id`) | Category classification.          |

### Notes

- **Amount Significance**: Positive amounts indicate deposits; negative amounts indicate withdrawals.
- **Foreign Key Constraints**:
    - `account_id` references `Account.account_id`.
    - `category_id` references `Category.category_id`.

---

## Budget Table

### Description

Defines budget limits and expectations for users, potentially per category and over specific time periods.

### Schema

| Field Name         | Data Type           | Constraints                  | Description                              |
|--------------------|---------------------|------------------------------|------------------------------------------|
| `budget_id`        | `SERIAL`            | Primary Key                  | Unique identifier for each budget.       |
| `user_id`          | `INTEGER`           | Foreign Key (`User.id`), Not Null | Owner of the budget.               |
| `category_id`      | `INTEGER`           | Foreign Key (`Category.category_id`), Optional | Category for the budget.    |
| `limit_amount`     | `DECIMAL(15, 2)`    | Not Null                     | Maximum spending limit.                  |
| `expectation_amount`| `DECIMAL(15, 2)`   | Optional                     | Expected spending or saving goal.        |
| `frequency`        | `FREQUENCY_ENUM`    | Not Null                     | Budget period (`Daily`, `Weekly`, `Monthly`, `Yearly`). |
| `start_date`       | `DATE`              | Optional                     | Budget period start date.                |
| `end_date`         | `DATE`              | Optional                     | Budget period end date.                  |

### Enumerations

- **FREQUENCY_ENUM**: An enumeration with values `'Daily'`, `'Weekly'`, `'Monthly'`, `'Yearly'`.

### Notes

- **Per-Category Budgets**: If `category_id` is set, the budget applies to that category; otherwise, it may apply to overall spending.
- **Date Fields**: `start_date` and `end_date` define custom budget periods if needed.

---

## Category Table

### Description

Provides a way to classify transactions and budgets into categories, allowing for detailed financial tracking and reporting.

### Schema

| Field Name    | Data Type         | Constraints                  | Description                              |
|---------------|-------------------|------------------------------|------------------------------------------|
| `category_id` | `SERIAL`          | Primary Key                  | Unique identifier for each category.     |
| `user_id`     | `INTEGER`         | Foreign Key (`User.id`), Optional | Owner of the category.               |
| `name`        | `VARCHAR(100)`    | Not Null                     | Category name.                           |
| `description` | `TEXT`            | Optional                     | Additional details about the category.   |

### Notes

- **User-Specific Categories**: If `user_id` is set, the category is user-specific; otherwise, it can be a global category.
- **Usage**: Categories are linked to transactions and budgets for better organization.

---

## Summary of Updates

1. **Primary Keys Added**: Introduced `id` fields as primary keys in tables where they were missing (`Transaction`, `Budget`, `Category`).

2. **Foreign Key Constraints**: Clearly defined relationships between tables using foreign keys to enforce referential integrity.

3. **Unique Constraints**:
    - Enforced uniqueness on `User.email` to prevent duplicate user accounts.
    - Added unique constraint on `Account.account_number` if applicable.

4. **Data Types and Fields**:
    - Replaced ambiguous field names with more descriptive ones (e.g., `TIME` to `created_at`).
    - Clarified `AMOUNT` in `Transaction` to allow both positive and negative values.
    - Added `description` fields for additional details where necessary.

5. **Enumerations**:
    - Defined enums for `ACCOUNT_TYPE` and `FREQUENCY` to standardize allowed values.
    - Recommended using PostgreSQL's `ENUM` type or lookup tables for better performance and data integrity.

6. **Timestamps**:
    - Added `created_at` and `updated_at` fields to track record creation and modification times.

7. **Password Security**:
    - Emphasized the importance of hashing passwords before storing them in the database.

8. **Normalization**:
    - Introduced a `Category` table to eliminate redundancy and provide better data organization.

9. **Additional Fields**:
    - Added optional fields like `first_name`, `last_name`, `currency`, `start_date`, and `end_date` for enhanced functionality.

---

