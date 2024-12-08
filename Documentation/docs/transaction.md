# Transaction Management

## Create New Transaction `POST`
#### API
```
/add_trans
```
#### Request
```json
{
    "email": "wick@example.com",
    "category_name": "food",
    "amount": 456.78,
    "notes": "winterlicious",
    "account_name": "td_debit"
}
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No user found for the provided email"
- Account not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No category found for the provided email"
- Category not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No account found for the provided email"
- Successfully added:
    - `STATUS_CODE`: `CREATED (200)`
    - `Message`: "Successfully added transaction"