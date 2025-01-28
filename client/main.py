import os
import sys

from flask import Flask
from flask_limiter import Limiter
from flask_limiter.util import get_remote_address
from flask_talisman import Talisman
import threading

sys.path.append(os.path.join(os.path.dirname(__file__)))

from utils import configify
from utils.coreUtils import isDictEx

from src.helpers.sendToServer import httpPostLoop
from src.helpers.mongoDb import mongoDbLoop
from src.helpers.localySave import locallySaveLoop

app = Flask(__name__)

_config = configify.get()

# Token Configuration
app.config["SECRET_KEY"] = _config["secretKey"]
app.config["AUTH_TOKEN"] = _config["oAuthToken"]

# Security configurations using Flask-Talisman
# Enforces HTTPS, sets security headers, etc.
Talisman(
    app,
    # force_https=True,  # Redirect HTTP to HTTPS
    strict_transport_security=True,  # Enable HSTS
    session_cookie_secure=True,  # Secure cookies
    content_security_policy={  # Define CSP
        "default-src": "'self'",
        "script-src": "'self'",
        "style-src": "'self'",
    },
)

# Rate limiting configuration using Flask-Limiter
# Limits requests to 100 per minute per IP address
limiter = Limiter(
    app=app, default_limits=["100 per minute"], key_func=get_remote_address
)

# Import routes from the routes folder
from src.routes import getRoutes

# Register routes
app.register_blueprint(getRoutes.routes)

# Create a thread to run the function
if isDictEx(_config, "server.status", "enabled"):
    httpPostThread = threading.Thread(
        target=httpPostLoop(_config["server"]["interval"])
    )
    httpPostThread.start()

if isDictEx(_config, "mongoDb.status", "enabled"):
    mongoDbThread = threading.Thread(target=mongoDbLoop(_config["mongoDb"]["interval"]))
    mongoDbThread.start()

if isDictEx(_config, "localFile.status", "enabled"):
    localySaveThread = threading.Thread(
        target=locallySaveLoop(_config["localFile"]["interval"])
    )
    localySaveThread.start()

app.run(host="0.0.0.0", port=8888, debug=True)
