# Database Initialization Tool

This directory contains utilities for initializing and managing the Notion database.

## Create Database Binary

The `create_database.rs` binary creates a new Notion database with the following properties:
- key (title field)
- datetime: Date field
- number: Number field 
- isWin: Checkbox field
- checked: Checkbox field

### Prerequisites

Before running this tool, make sure you have:

1. A Notion integration set up with the right permissions
2. A Notion page where the database will be created
3. The following environment variables set in your `.env` file:

```
NOTION_API_TOKEN=your_notion_integration_token
NOTION_PAGE_ID=id_of_page_where_database_will_be_created  # Optional if using --page-id flag
```

### Running the Tool

Run the database creation tool with:

```bash
# Create a database with default name "Spin Results Database"
cargo run --bin create_database

# Create a database with a custom name
cargo run --bin create_database -- --name "My Custom Database"

# Create a database on a specific page (overrides NOTION_PARENT_PAGE_ID)
cargo run --bin create_database -- --page-id your_page_id

# Create a database with both custom name and page ID
cargo run --bin create_database -- --name "My Custom Database" --page-id your_page_id
```

You can also use the short flags:
```bash
cargo run --bin create_database -n "My Custom Database" -p your_page_id
```

After successful execution, you'll get a database ID that should be added to your `.env` file:

```
NOTION_PAGE_ID=your_new_page_id
```

### Command-line Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --name` | Name of the database to create | "Spin Results Database" |
| `-p, --page-id` | Notion page ID where the database will be created | Value from NOTION_PAGE_ID env var |

## Note on Page IDs

To find page IDs in Notion:
1. Open a page in your browser
2. The ID is in the URL: `https://www.notion.so/{page_id}`
3. The page ID might contain hyphens which should be preserved