import json
import os

configFilePath = os.path.join(os.path.dirname(__file__), "..", "res")


# Function to read the config file
def get():
    configFilePath = os.path.join(os.path.dirname(__file__), "..", "config.json")
    with open(configFilePath, "r") as file:
        config = json.load(file)
    return config


# Function to write the modified config back to the file
def writeConfig(config):
    configFilePath = os.path.join(os.path.dirname(__file__), "..", "res")
    with open(configFilePath, "w") as file:
        json.dump(config, file, indent=4)


# Function to modify the config
def update(key, value):
    config = get()
    keys = key.split(".")
    current = config
    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]
    current[keys[-1]] = value

    writeConfig(current)
