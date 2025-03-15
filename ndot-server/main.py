import flask
import logging.config
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


if __name__ == "__main__":
    app.logger.info("Start ndot-server")
    app.run(debug=True, port=30080, host="0.0.0.0")
