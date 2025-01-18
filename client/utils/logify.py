import os
import datetime


def getLogFolderLocation():
    return os.path.join(os.path.dirname(__file__), "..", "res")


def getLogFileLocation(logFileName):
    return os.path.join(os.path.dirname(__file__), "..", "res", logFileName)


def createLogFile(logFileName):

    if not os.path.exists(getLogFolderLocation()):
        os.makedirs(getLogFolderLocation())

    if not os.path.exists(getLogFileLocation(logFileName)):
        # Write all lines back to the file
        with open(getLogFileLocation(logFileName), "w") as file:
            file.writelines("")


def getLogs(logFileName):

    createLogFile(logFileName)
    with open(getLogFileLocation(logFileName), "r") as file:
        return file.readlines()


def send(fileName, line, category="Information", location="Unknown"):

    lines = getLogs(fileName)

    time = datetime.datetime.now().strftime("%d-%m-%Y %H:%M:%S")
    username = os.getlogin()

    if len(lines) >= 10000:
        # Remove the oldest line (first line)
        lines.pop(0)

    # Append the new data
    lines.append(
        "Date&Time:"
        + time
        + " | Type:"
        + category
        + " | Location:"
        + location
        + " | User:"
        + username
        + " | Message:"
        + line
        + "\n"
    )

    # Write all lines back to the file
    with open(getLogFileLocation(fileName), "w") as file:
        file.writelines(lines)


def log(line, location="Unknown"):
    return send("application.log", line, "Information", location)


def error(line, location="Unknown"):
    return send("error.log", line, "Error", location)
