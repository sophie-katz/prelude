# MIT License
#
# Copyright (c) 2023 Sophie Katz
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

import datetime
import os
import pathlib
import re
import subprocess

# Constants
date_format                               = "%Y-%m-%d"
datetime_format                           = "%Y-%m-%d %H:%M:%S"

name_length_max                           = 19
version_length_max                        = 10
description_length_max                    = 26

days_threshold_ubuntu_version             = 180
days_threshold_nvm_version                = 30
days_threshold_node_version               = 30
days_threshold_yarn_version               = 30
days_threshold_yarn_lock_committed        = 30
days_threshold_cargo_lock_committed       = 30
days_threshold_apt_packages_last_upgraded = 30
days_threshold_docker_image_last_built    = 60

# Helper methods
def get_last_commit_date_of_file(path: str) -> datetime.date:
    return datetime.datetime.strptime(
        subprocess.run(
            [
                "git",
                "log",
                "-n",
                "1",
                "--pretty=format:%as",
                "--",
                path
            ], capture_output=True
        ).stdout.decode("UTF-8"),
        date_format
    ).date()

def get_apt_packages_last_upgraded() -> datetime.date:
    pattern_start_date = re.compile("Start-Date: (\d+-\d+-\d+).*")
    pattern_upgrade = re.compile("[ ]+upgrade[ ]*$")

    last_start_date = None

    with open("/var/log/apt/history.log") as file:
        for line in reversed(file.readlines()):
            match = re.match(pattern_start_date, line)
            if match:
                last_start_date = match[1]
                continue

            match = re.search(pattern_upgrade, line)
            if match:
                if last_start_date is None:
                    raise Exception("no start provided for upgrade command")

                return datetime.datetime.strptime(
                    last_start_date,
                    date_format
                ).date()

    raise Exception("no upgrade command in log")

def check_version(name: str, version: str, version_last_updated: datetime.date, days_threshold: int) -> bool:
    assert len(name) <= name_length_max, f"name cannot be longer than {name_length_max} characters"
    assert len(version) <= version_length_max, f"version cannot be longer than {version_length_max} characters"

    days_out_of_date = (datetime.date.today() - version_last_updated).days
    is_out_of_date = days_out_of_date > days_threshold

    message = ""
    message += f"  \033[1m{name}\033[0;0m version:"
    message += " " * (name_length_max - len(name))

    if is_out_of_date:
        message += "\033[1;31m"
    else:
        message += "\033[1;32m"

    message += version.ljust(version_length_max)

    message += f" \033[0;90m(last updated: {version_last_updated}, "

    if is_out_of_date:
        message += "\033[0;31mmust be updated\033[0;90m"
    else:
        message += "up to date"

    message += ")\033[0;0m"

    print(message)

    return not is_out_of_date

def check_date(description: str, last_updated: datetime.date, days_threshold: int) -> bool:
    assert len(description) <= description_length_max, f"description cannot be longer than {description_length_max} characters"

    days_out_of_date = (datetime.date.today() - last_updated).days
    is_out_of_date = days_out_of_date > days_threshold

    message = ""
    message += f"  \033[1m{description}\033[0;0m: "
    message += " " * (description_length_max - len(description))

    if is_out_of_date:
        message += "\033[1;31m"
    else:
        message += "\033[1;32m"

    message += f"{last_updated}"

    message += " \033[0;90m("

    if is_out_of_date:
        message += "\033[0;31mmust be updated\033[0;90m"
    else:
        message += "up to date"

    message += ")\033[0;0m"

    print(message)

    return not is_out_of_date

def get_env_safe(name: str) -> str:
    if not name in os.environ:
        raise Exception(f"environment variable not set: {name}")

    result = os.environ[name]

    if len(result) == 0:
        raise Exception(f"environment variable not set: {name}")

    return result

# Extract versions
ubuntu_version:              str           = get_env_safe("PORTOBELLO_UBUNTU_VERSION")
ubuntu_version_last_updated: datetime.date = datetime.datetime.strptime(get_env_safe("PORTOBELLO_UBUNTU_VERSION_LAST_UPDATED"), date_format).date()
nvm_version:                 str           = get_env_safe("PORTOBELLO_NVM_VERSION")
nvm_version_last_updated:    datetime.date = datetime.datetime.strptime(get_env_safe("PORTOBELLO_NVM_VERSION_LAST_UPDATED"), date_format).date()
node_version:                str           = get_env_safe("PORTOBELLO_NODE_VERSION")
node_version_last_updated:   datetime.date = datetime.datetime.strptime(get_env_safe("PORTOBELLO_NODE_VERSION_LAST_UPDATED"), date_format).date()
yarn_version:                str           = get_env_safe("PORTOBELLO_YARN_VERSION")
yarn_version_last_updated:   datetime.date = datetime.datetime.strptime(get_env_safe("PORTOBELLO_YARN_VERSION_LAST_UPDATED"), date_format).date()
yarn_lock_last_committed:    datetime.date = get_last_commit_date_of_file("/app/yarn.lock")
cargo_lock_last_committed:   datetime.date = get_last_commit_date_of_file("/app/Cargo.lock")
apt_packages_last_upgraded:  datetime.date = get_apt_packages_last_upgraded()
docker_image_last_built:     datetime.date = datetime.datetime.strptime(pathlib.Path("/home/dev/image/build_timestamp").read_text().strip(), datetime_format).date()

all_up_to_date = True
update_instructions = []

print("Versions:")

if not check_version("Ubuntu", ubuntu_version, ubuntu_version_last_updated, days_threshold_ubuntu_version):
    all_up_to_date = False
    update_instructions.append("""To upgrade Ubuntu version:

- Go to https://hub.docker.com/_/ubuntu to find the latest Ubuntu Docker image
- Edit Dockerfile.dev
- Change the FROM image to the new image tag
- Also update the PORTOBELLO_UBUNTU_VERSION and PORTOBELLO_UBUNTU_VERSION_LAST_UPDATED variables below""")

if not check_version("NVM", nvm_version, nvm_version_last_updated, days_threshold_nvm_version):
    all_up_to_date = False
    update_instructions.append("""To upgrade NVM:

- Go to https://github.com/nvm-sh/nvm/releases to find the latest NVM release
- Edit Dockerfile.dev
- Update the PORTOBELLO_NVM_VERSION and PORTOBELLO_NVM_VERSION_LAST_UPDATED variables""")

if not check_version("Node.JS", node_version, node_version_last_updated, days_threshold_node_version):
    all_up_to_date = False
    update_instructions.append("""To upgrade Node.JS:

- Run 'nvm version-remote --lts' to get the latest LTS release of Node.JS
- Edit Dockerfile.dev
- Update the PORTOBELLO_NODE_VERSION and PORTOBELLO_NODE_VERSION_LAST_UPDATED variables""")

if not check_version("Yarn", yarn_version, yarn_version_last_updated, days_threshold_yarn_version):
    all_up_to_date = False
    update_instructions.append("""To upgrade Yarn:

- Go to https://github.com/yarnpkg/berry/releases to find the latest Yarn release
- Edit Dockerfile.dev
- Update the PORTOBELLO_YARN_VERSION and PORTOBELLO_YARN_VERSION_LAST_UPDATED variables""")

print()
print("Last updated:")

if not check_date("Cargo lock last committed", cargo_lock_last_committed, days_threshold_cargo_lock_committed):
    all_up_to_date = False
    update_instructions.append("""To Cargo lock:

- Run 'cargo update'
- Commit changes to cargo.lock""")

if not check_date("Yarn lock last committed", yarn_lock_last_committed, days_threshold_yarn_lock_committed):
    all_up_to_date = False
    update_instructions.append("""To upgrade Yarn lock:

- Run 'yarn install'
- Commit changes to yarn.lock""")

if not check_date("Apt packages last upgraded", apt_packages_last_upgraded, days_threshold_apt_packages_last_upgraded):
    all_up_to_date = False
    update_instructions.append("""To upgrade Apt packages:

- Open a terminal on the host machine, NOT within the Docker environment in VS Code
- Run 'docker exec -it -u 0 docker_dev_1 /bin/bash'
    - This may have to be run as root on the host machine
    - 'docker_dev_1' might need to be replaced by another running container name
    - Use 'docker ps' to just running containers""")

if not check_date("Docker image last built", docker_image_last_built, days_threshold_docker_image_last_built):
    all_up_to_date = False
    update_instructions.append("""To rebuild Docker image:

- In VS Code, click 'Dev Contaner' button on the bottom left of the window
- Select 'Rebuild Container'""")

if not all_up_to_date:
    for i in update_instructions:
        print()
        print(i)

    os._exit(1)
