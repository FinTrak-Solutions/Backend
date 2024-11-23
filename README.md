# Backend


### Update Documentation
```shell
# install env
cd Backend
virtualenv venv
source venv/bin/activate
pip3 install mkdocs
# goto dir
cd Documentation
# check, update locally
mkdocs serve
# build and deploy
mkdocs build
mkdocs gh-deploy
```