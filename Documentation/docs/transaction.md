# Transaction Management

## Create New Transaction `POST`
#### API
```
/transaction_create
```
#### Request
```json
{
    "email": "wick@example.com",
    "category_type": "weapon",
    "account_name": "td_credit",
    "amount": 987.654,
    "notes": "hello world"
}
```