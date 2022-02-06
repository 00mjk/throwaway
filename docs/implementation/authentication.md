# Authentication
Signed (then Encrypted ?) JWTs.

Create Profile

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

Get Token

```
BASIC=$(echo -n 'test-1@domain.test:test1' | base64)
TOKEN=$(curl \
  --silent \
  --request POST \
  --header 'Content-Type: application/json' \
  --header "Authorization: Basic ${BASIC}" \
  --data-raw '{
    "lifespan": 600,
    "attributes": {
      "profile": {
        "create": true
      }
    }
  }' \
  http://localhost:8000/token | jq -r .token)

echo "Token: ${TOKEN}"
```

Use Token

```
curl \
  --silent \
  --request GET \
  --header 'Content-Type: application/json' \
  --header "Authorization: Bearer ${TOKEN}" \
  http://localhost:8000/token/info | jq .
```
