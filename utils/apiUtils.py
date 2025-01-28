import jwt
import os
import sys
import string
import random

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from utils import configify


def getToken(hostId: str):
    _config = configify.get()
    payload = {"hostId": hostId, "token": _config["oAuthToken"]}

    return jwt.encode(payload, _config["secretKey"], algorithm="HS256")


def genCreds():
    # Define the characters to choose from
    characters = string.ascii_letters + string.digits + string.punctuation

    # Generate the random string
    token = "".join(random.choice(characters) for _ in range(20))
    secretKey = "".join(random.choice(characters) for _ in range(10))

    return token, secretKey
