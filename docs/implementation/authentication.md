# Authentication
Signed (then Encrypted ?) JWTs.

```
curl \
  --silent \
  --request POST \
  --header 'Content-Type: application/json' \
  --data-raw '{
    "name": "Test 1",
    "email": "test-1@domain.test",
    "password": "test1",
    "country": "UK",
    "timezone": "GMT"
  }' \
  http://localhost:8000/register | jq .
```

```
TOKEN=$(curl \
  --silent \
  --request POST \
  --header 'Content-Type: application/json' \
  --data-raw '{
    "email": "test-1@domain.test",
    "password": "test1"
  }' \
  http://localhost:8000/token | jq -r .token)

echo "Token: ${TOKEN}"
```

```
curl \
  --silent \
  --request PATCH \
  --header 'Content-Type: application/json' \
  --header "Authorization: Bearer ${TOKEN}" \
  --data-raw '{"name": "Updated"}' \
  http://localhost:8000/profile | jq .
```
