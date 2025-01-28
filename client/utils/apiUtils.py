from functools import wraps
from flask import request, jsonify


# Middleware to validate authorization token
def tokenRequired(app, f):
    @wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("Authorization")
        if not token or token != f"Bearer {app.config['AUTH_TOKEN']}":
            return jsonify({"message": "Unauthorized"}), 401
        return f(*args, **kwargs)

    return decorated
