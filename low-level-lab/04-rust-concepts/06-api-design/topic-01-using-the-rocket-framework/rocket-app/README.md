Endpoints

### CRUD

#### Create
POST

#### Read
GET
GET

#### Update
PUT

#### Delete
DELETE


Routes

_______________________________________________________________________________

./httpie-api-testing/
.
├── 01-create
│   └── post_rustaceans.sh
├── 02-read
│   ├── get_rustaceans_id.sh
│   └── get_rustaceans.sh
├── 03-update
│   └── put_rustaceans_id.sh
└── 04-delete
    └── delete_rustaceans_id.sh

To send requests with HttPie

First ensure that you are in the route of the project directory
then run this:

find httpie-api-testing/ -type f -name "*.sh" -exec chmod +x {} \;

_______________________________________________________________________________

E.g. Send

```sh
./httpie-api-testing/01-create/post_rustaceans.sh
```
_______________________________________________________________________________
