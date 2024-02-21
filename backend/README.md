# Cipherly Backend Server

1. Prerequisites:

   - [Homebrew](https://brew.sh/)
     ```sh
     /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
     ```
   - [pipx](https://github.com/pypa/pipx)
     ```sh
     brew install pipx
     ```
   - [poetry](https://python-poetry.org/docs/#installation)
     ```
     pipx install poetry
     ```

1. Change to this folder (`backend`).

   ```sh
   cd backend
   ```

1. Install the dependencies.

   ```sh
   poetry install
   ```

1. Create and download the Service Account credentials file from Google Cloud Console.

   [Python FastAPI Dev Server Service Account](https://console.cloud.google.com/iam-admin/serviceaccounts/details/110996570305890367782/keys?project=secret-cipher-413823)

1. Install the credential into the backend server.

   ```sh
   mkdir .secrets
   mv ~/Downloads/secret-cipher-*.json .secrets/credentials.json
   ```

1. Start the server.

   ```sh
   export $(cat .env | xargs)
   poetry run uvicorn cipherly:app --reload
   ```
