# Backend Documentation
## API
### [Authentication](authentication.md)
| API                                        | Done | Time Finished     | Link to Docs      |
|--------------------------------------------|---------|-------------------|-------------------|
| `/signup`                                  | ✅       | 2024-12-07 2:00pm | [View Docs](authentication/#signup-post) |

### [Account Management](account.md)
| API                                        | Done | Time Finished     | Link to Docs      |
|--------------------------------------------|---------|-------------------|-------------------|
| `/account_create`                          | ✅       | 2024-12-07 3:10pm | [View Docs](account/#create-new-account-for-user-post) |
| `/account_summary?email=<>`                | ✅       | 2024-12-07 3:40pm | [View Docs](account/#get-account-overview-for-user-get) |
| `/delete_account?email=<>&account_name=<>` | ✅       | 2024-12-07 4:20pm | [View Docs](account/#delete-an-account-for-user-delete) |
| `/account_detail`                          |         | TBD               | [View Docs](account/#get-account-detailed-view-for-user-get) |

### [Category Management](category.md)
TBD

### [Transaction Management](transaction.md)
TBD

## Postman API testing
Check out the postman API documentation [here](https://web.postman.co/workspace/46a5447a-bfb7-47fa-8a8b-0da03a25416e/collection/40276125-9521e786-da55-44fd-9b33-98f4b67d293e) (localhost version)





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