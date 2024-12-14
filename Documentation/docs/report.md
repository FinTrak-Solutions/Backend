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
- Successfully extracted:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<String>>`: Example output
```
[
    "Category Summary:",
    "clothes : 1370.34",
    "food : 3751.419999999999",
    "Account Summary:",
    "td_credit: 5121.759999999999"
]
```

## Report Details `GET`
#### API
```
/report_details?email=<>
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Json<Vec<CategorySummary>>`: Empty
- Successfully extraced:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<CategorySummary>>`: Each CategorySummary is in the following format:
    ```
    pub struct CategorySummary {
        pub nickname: String,
        pub budget: f64,
        pub budget_freq: String,
        pub overbudget: bool,
        pub amount: f64,
        pub cat_trans: Vec<String>,
    }
    ```
    - An example response is provided below:
    ```
    [
        {
            "nickname": "food",
            "budget": 100.0,
            "budget_freq": "weekly",
            "overbudget": true,
            "total": 3739.119999999999,
            "cat_trans": [
                "2024-12-08 05:07:18.906680 UTC, 456.78, winterlicious",
                "2024-12-08 05:31:26.645759 UTC, 100, EMT",
                "2024-12-08 05:31:29.750300 UTC, 100, EMT",
                "2024-12-08 05:31:33.449547 UTC, 100, EMT",
                "2024-12-08 05:31:34.869775 UTC, 100, EMT",
                "2024-12-08 05:32:39.826673 UTC, 200, EMT",
                "2024-12-08 05:56:55.727 UTC, 200, EMT",
                "2024-12-08 05:56:57.048726 UTC, 200, EMT",
                "2024-12-08 05:56:57.971029 UTC, 200, EMT",
                "2024-12-11 20:49:07.598004 UTC, 4, grocery",
                "2024-12-11 20:55:42.214059 UTC, 354, grocery",
                "2024-12-11 20:55:43.662435 UTC, 354, grocery",
                "2024-12-13 03:28:03.679604 UTC, 456.78, winterlicious",
                "2024-12-13 03:28:05.358450 UTC, 456.78, winterlicious",
                "2024-12-13 03:28:06.424495 UTC, 456.78, winterlicious"
            ]
        },
        {
            "nickname": "clothes",
            "budget": 12345.678,
            "budget_freq": "monthly",
            "overbudget": false,
            "total": 1370.34,
            "cat_trans": [
                "2024-12-11 20:39:56.714375 UTC, 456.78, Uniqlo",
                "2024-12-11 20:39:58.473895 UTC, 456.78, Uniqlo",
                "2024-12-11 20:40:00.615091 UTC, 456.78, Uniqlo"
            ]
        }
    ]
    ```