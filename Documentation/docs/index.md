# Backend Documentation
## API
### [Authentication](authentication.md)
### [Account Management](account.md)

| Category          | API                                       | Done | Time Finished     | Link to Docs                                                            |
|-------------------|-------------------------------------------|---------|-------------------|-------------------------------------------------------------------------|
| Authentication    | `/signup`                                 | ✅       | 2024-12-07 2:00pm | [View Docs](authentication/#signup-post)                                |
| Account Management| `/account_create`                         | ✅       | 2024-12-07 3:10pm | [View Docs](account/#create-new-account-for-user-post)                  |
| Account Management| `/account_summary?email=<user@email>` | ✅       | 2024-12-07 3:40pm | [View Docs](account/#get-account-overview-for-user-get)      |
| Account Management| `/account_detail`                         |         |                   | [View Docs](account/#get-account-detailed-view-for-user-get) |





## Quick Start
### Update Documentation
#### Clone project
```shell
git clone https://github.com/FinTrak-Solutions/Backend.git
```
#### Install virtual env
```shell
cd Backend
virtualenv venv
source venv/bin/activate
pip3 install mkdocs
```
#### Goto dir
```shell
cd Documentation
```
#### Modify .md in docs/
```shell
# check, update locally
mkdocs serve
# modify .md in docs/
```
#### Build and deploy
```shell
# build and deploy
mkdocs build
mkdocs gh-deploy
```

## Database Schema
Can be found [here](database.md)

## Project layout
TBD