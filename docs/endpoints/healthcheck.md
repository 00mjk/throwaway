# `/healthcheck`

https://tools.ietf.org/id/draft-inadarei-api-health-check-05.html#section-5
CHECK:
    1. Host stats (disk space, memory, CPU etc.)
    2. Cache/DB/service connectivity (speed of response etc.)
    3. DB migration status (last applied etc.)
    4. Last service restart?
    5. What version is being run.
    6. Host ID? i.e. what box is this running on (hostname?)
NOTE: Healthcheck shouldn't duplicate metrics job

```
curl \
  --silent \
  --request GET \
  --header 'Content-Type: application/json' \
  http://localhost:8000/health | jq .
```
