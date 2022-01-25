# Architecture
Implementation decisions.

TODO: Split between code (internal) and access (external) decisions?

## Authentication
* JWTs Token ?
* API Key ?

### JWT Scopes
* Github style, only allow actions on certain objects

### Lock Out
* Lock out after failed login attempts

### REST (actual REST that is)
...

### 2FA / Customer Security
...

### Beta Features / Flags
...

### Licensing
* Payable add-ons, limits etc.

### Shadowed Enablement / Dark Launch / Staged Roll-out
...

### OAuth / OpenID Connect
...

### GraphQL
...

### Notifications
* Email
* Push
* RSS
* WebHooks

### Deprecations
...

## Authorization
...

## Caching / Cache
...

## Metrics
...

## Task Scheduling
* Celery
* Async

## IP
* IpV4
* IpV6

* HTTPS
* Block HTTP

## Compression
* Brotli
* GZip

## CORS Tokens
...

## Single Responsibility Principle
...

## Observer Pattern (?)
- Event based processing

## Session Management
- Storing
- Invalidating
- Invalidate sessions using endpoint?

## Rate Limiting
...

## Config
...

## Secrets
...

## Databases / State Management
...

## Migrations

## Error Handling
* Concise messaging

## Locale (i18n)
...

## Logging
...

## Audit
- Think about a logging standard

## Dependency Injection

## API Versioning
- https://stripe.com/blog/api-versioning

## Pagination
...

## Filters
...

## Sorting
...

## Signed URLs
...

## Documentation
MDBook - why generated docs aren't enough.

## Idempotent Requests
...

## Events
...

## Tracing
...

## Access Control
- RBAC/ABAC
- Hierarchical

## Multi-tenancy
...

## Presentation
...

## Profiling / Performance
...

## Metadata
Ask for provided data to be returned as a tagging functionality.

## Websockets
...

## Timezone
* Store everything as UTC, expose as local timezone?

## Human vs Machine - Friendly Balance
...

## Dry Run Requests
...

## CLI Interface
Showcase generated CLI capabilities.

## Terraform Module Interface
Showcase generated Terraform Module capabilities.

## Support federated authentication (OIDC / SAML)
...

## Internal Endpoints
...

## Internal vs External API
- Internal, not necessary 'private' - just no guarantee of versioning, change at any time ...

## Tests
- Unit
- Mutation
- E2E

## Other
Do we allow nested endpoints? `/balance/history` ?
Would interp well with CLI I suppose.

Two viewpoints to consider. (stories ?)
* Business
* Customer

Business will need to manage quite a lot of resources.

Customer will simply provide details in a request.

Plan out an end-to-end process, design through `curl` commands?

Is there a 'dev' mode for Stripe I could test.

Create database schema first (and nail relationships)

Should database tables and API routes be plural?

Do we allow one 'account' to manage multiple businesses?
