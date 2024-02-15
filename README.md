# Secret Cipher

A web app for sharing secrets.

Development of this app is being presented as a series of videos on [The Friendly TL's YouTube channel](https://www.youtube.com/@FriendlyTL).

## Setting up the Development Environment

1. Clone the repository

   ```sh
   git clone git@github.com:shadanan/secret-cipher.git
   cd secret-cipher
   ```

### Backend Dev Server

1. Change to the `backend` folder.

   ```sh
   cd backend
   ```

1. Create and activate a virtual environment.

   ```sh
   python3 -m venv .venv
   source .venv/bin/activate
   ```

1. Install dependencies.

   ```sh
   pip install -r requirements.txt
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
   uvicorn main:app --reload
   ```

### Frontend Dev Server

1. Change to the `frontend` folder.

   ```sh
   cd frontend
   ```

1. Install NPM dependencies.

   ```sh
   npm install
   ```

1. Run the dev server.

   ```sh
   npm run dev
   ```
