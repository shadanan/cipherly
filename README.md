# Cipherly

A web app for sharing secrets.

Development of this app is being presented as a series of videos on [The Friendly TL's YouTube channel](https://www.youtube.com/@FriendlyTL).

## Prod

```sh
# Build prod container
docker build -t gcr.io/cipherly/cipherly .

# Run prod container at http://127.0.0.1:8000
docker run -p 8000:8000 gcr.io/cipherly/cipherly

# Deploy to staging
./deploy.sh

# Deploy to prod
./deploy.sh --prod
```

## Message Format

### URL Form

The basic URL form for a cipherly message is:

```
/schema/<schema>/version/<version>/<header>.<envelope>#<header>.<secret>
```

The schema is a name for the secret type. Currently only `password` is supported.

The version represents what version the secret type is on. Should be 1.

The header is a URL-safe base64 encoded value that is specific to the schema.

#### Password

For the password schema encrypted message:

```
/schema/password/version/1/<header>.<envelope>#<secret>
```

## Setting up the Development Environment

1. Clone the repository

   ```sh
   git clone git@github.com:shadanan/secret-cipher.git
   cd secret-cipher
   ```

1. Follow backend dev instructions in [README.md](backend/README.md).

1. Follow frontend dev instructions in [README.md](frontend/README.md).
