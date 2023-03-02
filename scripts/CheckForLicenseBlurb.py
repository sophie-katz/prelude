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

import os

IGNORED = [
    ".d.ts",
    "/.cargo/",
    "/.devcontainer/",
    "/.eslintrc.js",
    "/.quasar/",
    "/.stoplight/",
    "/.vscode/",
    "/.yarn/",
    "/api-bindings-client-typescript-fetch/apis/",
    "/api-bindings-client-typescript-fetch/index.ts",
    "/api-bindings-client-typescript-fetch/models/",
    "/api-bindings-client-typescript-fetch/runtime.ts",
    "/db/migration/src/lib.rs",
    "/db/migration/src/main.rs",
    "/db/src/entities",
    "/jest.config.js",
    "/migration/README.md",
    "/node_modules/",
    "/quasar.config.js",
    "/router/index.ts",
    "/stores/index.ts",
    "/target/"
]

EXTENSIONS = [
    ".bash",
    ".env",
    ".html",
    ".js",
    ".md",
    ".py",
    ".rs",
    ".scss",
    ".toml",
    ".ts",
    ".vue",
    ".yml"
]

INCLUDED = [
    "/.env.example.dev",
    "/Dockerfile.dev"
]

def is_ignored(file):
    for i in IGNORED:
        if i in file:
            return True
    
    return False

def has_included_extension(file):
    for i in EXTENSIONS:
        if file.endswith(i):
            return True
    
    return False

def is_explicitly_included(file):
    for i in INCLUDED:
        if i in file:
            return True
    
    return False

def is_included(file):
    if is_ignored(file):
        return False
    
    if has_included_extension(file):
        return True
    
    if is_explicitly_included(file):
        return True
    
    return False

def enumerate_files_to_check():
    for root, dirs, files in os.walk("/app"):
        for file in files:
            absolute = os.path.join(root, file)

            if is_included(absolute):
                yield absolute

def has_license_blurb(file):
    with open(file) as handle:
        text = handle.read()

        if not "Copyright (c) " in text:
            return False
        
        if not "MIT License" in text:
            return False
        
        return True
            
first = True

for i in enumerate_files_to_check():
    if not has_license_blurb(i):
        if first:
            print("License blurb must be added to:")
            first = False

        print(f"  {i}")

if first:
    print("All files have license blurb")
else:
    os._exit(1)
