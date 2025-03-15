# ndot-server API Documentation

The ndot-server is a Flask-based web server that hosts the ndot-client web application and provides API endpoints for saving and retrieving DOT files.

## API Endpoints

### Web Application Routes

#### `GET /`

Redirects to the `/ndot` endpoint.

**Response:**
- Status Code: 302 (Found)
- Redirects to: `/ndot`

#### `GET /ndot`

Serves the main web application.

**Response:**
- Status Code: 200 (OK)
- Content: HTML file from the static directory (`static/index.html`)

#### `GET /ndot/<path:path>`

Serves static assets for the web application.

**Parameters:**
- `path`: The path to the asset

**Response:**
- Status Code: 200 (OK)
- Content: The requested asset file

### Data API Endpoints

#### `POST /api/save`

Saves DOT content with a specific ID.

**Request Body:**
```json
{
  "id": "unique-file-id",
  "content": "digraph { a -> b; }"
}
```

**Response (Success):**
- Status Code: 200 (OK)
```json
{
  "success": true,
  "message": "File saved successfully"
}
```

**Response (Error):**
- Status Code: 400 (Bad Request) - If the request format is invalid
- Status Code: 500 (Internal Server Error) - If there's an error saving the file
```json
{
  "success": false,
  "error": "Error message"
}
```

#### `GET /api/get/<id>`

Retrieves DOT content by ID.

**Parameters:**
- `id`: The unique identifier of the DOT file

**Response (Success):**
- Status Code: 200 (OK)
```json
{
  "success": true,
  "content": "digraph { a -> b; }"
}
```

**Response (Error):**
- Status Code: 404 (Not Found) - If the file is not found
- Status Code: 500 (Internal Server Error) - If there's an error retrieving the file
```json
{
  "success": false,
  "error": "Error message"
}
```

## Server Configuration

The server can be configured using command-line arguments:

- `--port`: Port to run the server on (default: 30080)
- `--save-dir`: Directory to save DOT files (default: "saved_data")

Example:
```bash
python main.py --port 8080 --save-dir "my_dot_files"
```
