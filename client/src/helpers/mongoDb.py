import time
import os
import sys

sys.path.append(os.path.join(os.path.dirname(__file__), "..", ".."))

from utils import configify, logify
from src import getInfo


# Function to send a POST request
def saveInMongoDb():

    _config = configify.get()
    data = getInfo.get()
    return True


# Timer-based loop to send POST requests
def mongoDbLoop(interval=60):
    while True:
        saveInMongoDb()
        time.sleep(interval)
