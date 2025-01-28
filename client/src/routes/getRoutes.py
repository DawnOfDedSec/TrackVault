import os
import sys

from flask import Blueprint, jsonify

sys.path.append(os.path.join(os.path.dirname(__file__)))

from src import getInfo

from utils.coreUtils import tokenRequired
from utils import configify

routes = Blueprint("getRoutes", __name__)

_config = configify.get()


@routes.route("/info", methods=["GET"])
@tokenRequired
def get_data():
    info = getInfo.get()
    return jsonify({"identity": _config["hostInfo"], "metadata": info}), 200


# Health check endpoint
@routes.route("/health", methods=["GET"])
@tokenRequired
def health_check():
    return jsonify({"identity": _config["hostInfo"], "status": "healthy"}), 200
