# Notion CRUD API with Rust and Axum

A clean architecture implementation of a REST API that interfaces with Notion's Database API, built with Rust and Axum framework.

## Features

- Full CRUD operations for Notion database entries
- Clean Architecture implementation
- Type-safe API with Rust
- CORS support for frontend integration
- Error handling and logging
- Environment-based configuration

## Project Structure 
```
notion-crud/
├── src/
│ ├── domain/ # Business logic and interfaces
│ ├── application/ # Use cases
│ ├── infrastructure/ # External implementations
│ └── api/ # HTTP layer
```


## Prerequisites

- Rust (latest stable version)
- Notion API Integration Token
- Notion Database ID

## Environment Variables

Create a `.env` file in the root directory:

```
NOTION_API_KEY=your_notion_api_key
NOTION_DATABASE_ID=your_notion_database_id
DAILY_SPIN_LIMIT=3 # Optional: defaults to 1
```


## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/notion-api-rust.git
cd notion-api-rust
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```


The server will start at `http://localhost:3000`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/spin-results` | Create a new entry |
| GET | `/spin-results` | Get all entries |
| PUT | `/spin-results/:page_id` | Update an entry |
| DELETE | `/spin-results/:page_id` | Delete an entry |

## Request/Response Format

### Create/Update Entry

```json
{
"name": "John Doe",
"phone_number": "+1234567890",
"ticket": 123,
"reward": "Prize A"
}
```


## Error Handling

The API returns appropriate HTTP status codes:
- 200: Success
- 201: Created
- 204: No Content (for successful deletion)
- 500: Internal Server Error

## Architecture

This project follows Clean Architecture principles:

1. **Domain Layer**: Core business logic and interfaces
   - Models
   - Repository traits
   - Error types

2. **Application Layer**: Use cases
   - Services implementing business logic
   - Orchestration of domain objects

3. **Infrastructure Layer**: External implementations
   - Notion API client
   - Database interactions

4. **API Layer**: HTTP concerns
   - Route definitions
   - Request/Response handling
   - CORS configuration

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Notion API](https://developers.notion.com/) - Database API
- Clean Architecture principles by Robert C. Martin