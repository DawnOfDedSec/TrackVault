import json
import os

configFilePath = os.path.join(os.path.dirname(__file__), "..", "res", "config.json")


# Function to read the config file
def get():
    with open(configFilePath, "r") as file:
        config = json.load(file)
    return config


# Function to write the modified config back to the file
def writeConfig(config):
    with open(configFilePath, "w") as file:
        json.dump(config, file, indent=4)


# Function to modify the config
def modify(key, value):
    config = get()
    keys = key.split(".") if isinstance(key, str) else key
    current = config

    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]

    current[keys[-1]] = value
    return writeConfig(config)


def update(config):
    return writeConfig(config)
