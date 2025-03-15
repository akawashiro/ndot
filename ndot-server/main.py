import argparse
import flask
import logging.config
import os
import json
from flask_cors import CORS, cross_origin

STATIC_DIR = "static"

# See https://flask.palletsprojects.com/en/stable/logging/#basic-configuration
logging.config.dictConfig(
    {
        "version": 1,
        "formatters": {
            "default": {
                "format": "%(asctime)s %(levelname)8s %(filename)12s:%(lineno)04d %(message)s",
            }
        },
        "handlers": {
            "wsgi": {
                "class": "logging.StreamHandler",
                "stream": "ext://flask.logging.wsgi_errors_stream",
                "formatter": "default",
            }
        },
        "root": {"level": "DEBUG", "handlers": ["wsgi"]},
    }
)


app = flask.Flask(__name__)
cors = CORS(app)  # allow CORS for all domains on all routes.
app.config["CORS_HEADERS"] = "Content-Type"


@app.route("/ndot", defaults={"path": ""})
@app.route("/ndot/<path:path>")
@cross_origin()
def dashboard(path: str) -> flask.Response:
    if path.startswith("assets"):
        return flask.send_from_directory(STATIC_DIR, path)
    return flask.send_from_directory(STATIC_DIR, "index.html")


@app.get("/")
@cross_origin()
def index():
    return flask.redirect("/ndot", code=302)


@app.route("/api/save", methods=["POST"])
@cross_origin()
def save_dot():
    try:
        data = flask.request.get_json()

        if not data or "id" not in data or "content" not in data:
            return flask.jsonify(
                {
                    "success": False,
                    "error": "Invalid request format. 'id' and 'content' fields are required.",
                }
            ), 400

        file_id = data["id"]
        content = data["content"]

        # Ensure the save directory exists
        save_dir = flask.current_app.config["SAVE_DIR"]
        os.makedirs(save_dir, exist_ok=True)

        # Save the content to a file with the id as the filename
        file_path = os.path.join(save_dir, file_id)
        with open(file_path, "w") as f:
            f.write(content)

        return flask.jsonify({"success": True, "message": "File saved successfully"})

    except Exception as e:
        app.logger.error(f"Error saving file: {str(e)}")
        return flask.jsonify(
            {"success": False, "error": f"Error saving file: {str(e)}"}
        ), 500


@app.route("/api/get/<id>", methods=["GET"])
@cross_origin()
def get_dot(id):
    try:
        save_dir = flask.current_app.config["SAVE_DIR"]
        file_path = os.path.join(save_dir, id)

        if not os.path.exists(file_path):
            return flask.jsonify({"success": False, "error": "File not found"}), 404

        with open(file_path, "r") as f:
            content = f.read()

        return flask.jsonify({"success": True, "content": content})

    except Exception as e:
        app.logger.error(f"Error retrieving file: {str(e)}")
        return flask.jsonify(
            {"success": False, "error": f"Error retrieving file: {str(e)}"}
        ), 500


def main():
    # Parse command line arguments
    parser = argparse.ArgumentParser(description="ndot-server")
    parser.add_argument(
        "--port",
        type=int,
        default=30080,
        help="Port to run the server on (default: 30080)",
    )
    parser.add_argument(
        "--save-dir",
        type=str,
        default="saved_data",
        help="Directory to save dot files (default: saved_data)",
    )
    args = parser.parse_args()

    # Set the save directory in the app config
    app.config["SAVE_DIR"] = args.save_dir

    # Create the save directory if it doesn't exist
    os.makedirs(args.save_dir, exist_ok=True)

    # Start the server
    app.logger.info(f"Start ndot-server on port {args.port}")
    app.logger.info(f"Using save directory: {args.save_dir}")
    app.run(debug=True, port=args.port, host="0.0.0.0")


if __name__ == "__main__":
    main()
