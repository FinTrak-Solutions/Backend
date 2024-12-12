# Report and Analysis
## Report Overview `GET`
#### API
```
/report_overview?email=<>
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Json<Vec<String>>`: Empty
- Successfully added:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<String>>`: Json<["Account Summary:", "td_debit: -$3200", "Category Summary: ", "food: -$200", "clothes: -$300"]>