# Authentication

## Signup `POST`
#### API
```
http://127.0.0.1:8000/signup
```
#### Post Body (_Json_)
- Request:
```Json
{
    "username": "John Wick",
    "email": "test@example.com",
    "password": "123456"
}
```
- Response: (String or Status Code?)
    - If email exists:
        - password good: `STATUS_CODE::OK` (200)
        - password bad: `STATUS_CODE::BAD_REQUEST` (400)
    - If email does not exist: `STATUS_CODE::CREATED` (201)

# Account Management
## Create New Account for User (POST)
#### API
```
http://127.0.0.1:8000/account_create
```
- Request:
```Json
{
    "email": "test@example.com",
    "type": "credit",
    "account_name": "td_credit",
    "init_balance(optional)": -200
}
```
- Response:
    - Successfully created: `STATUS_CODE::CREATED` (201)
    - Failed to create (e.g. account name already exists): `STATUS_CODE::BAD_REQUEST` (400)

## Get Account Overview for User (GET)
#### API
```
http://127.0.0.1:8000/account_summary
```
- Request:
```Json
{
    "email": "test@example.com"
}
```
- Response:
```Json
{
    "account1": "888.88CAD",
    "account2": "666.66CAD"
}
```

## Get Account Detailed View for User (GET)
#### API
```
http://127.0.0.1:8000/account_details
```
- Request:
```Json
{
    "email": "test@example.com",
    "account_name": "td_credit"
}
```
- Response:
```Json
[
    {
        "1": "TBD"
    },
    {
        "2": "TBD"
    }
]
```
