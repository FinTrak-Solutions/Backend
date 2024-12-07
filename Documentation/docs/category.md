# Category Management

## Create New Category `POST`
#### API
```
/category_create
```
#### Request
```json
{
    "email": "wick@example.com",
    "nickname": "TBD",
    "category_type": "weapon",
    "budget": 12345.678,
    "budget_freq": "daily"
}
```
#### Response:
- Successfully created: 
    - `STATUS_CODE`: `CREATED (201)` 
    - `Message`: "Successfully created `category_nickname`"
- Failed to create: 
    - No `email` found in `user` table
        - `STATUS_CODE`: `BAD_REQUEST (400)`
        - `Message`: "No user found for the provided email"
    - `category_nickname` already exists for current `user`
        - `STATUS_CODE`: `BAD_REQUEST (400)` 
        - `Message`: Failed to create new category: duplicate nicknames

## Get Category Overview for User `GET`
#### API
```
/category_summary?email=wick@example.com
```
#### Response:
TBD

## Delete a category for User `DELETE`
#### API
```
/delete_category?email=<>&category_type=<>
```
#### Response:
TBD

## Update a category for User `UPDATE`
#### API
```
/category_update
```
#### Request
```json
{
    "email": "wick@example.com",
    "nickname": "TBD",
    "category_type": "weapon",
    "budget": 12345.678,
    "budget_freq": "daily"
}
```
#### Response:
TBD