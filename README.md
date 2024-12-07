# FinTrack Backend

## [Online Documentation](https://fintrak-solutions.github.io/Backend/)

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

## Create new database using `PostgreSQL`
```shell
psql -U postgres

# PostgreSQL prompt
CREATE DATABASE financial_tracker_db;
# Verify
\l 
# Exit
\q
```