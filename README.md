# Rideboard V2

A Proper Rideboard.

## Features

- Blazingly Fast API in Rust
- Dynamic Vue Frontend
- PostgreSQL Database

## Development Setup

### Requirements

- Rust (I use 1.81)
- npm
- Ideally docker to spin up a database to test on

### Steps

#### Database Setup (Docker)

`cd $PROJECT_ROOT`

`docker run -it --rm -e 5432:5432 -d ./src/migrations:/docker-entrypoint-initdb.d -e POSTGRES_USER=rideboard -e POSTGRES_DATABASE=rideboard -e POSTGRES_PASSWORD=supersecurepassword postgres`

#### Setup .env

1. Copy `.env.example` as `.env`

2. Fill in data fields.
  - For `DATABASE_URL`, format is `postgresql://USERNAME:PASSWORD@HOST/DATABASE`
  - Contact an RTP for CSH Auth Credentials.
  - Create a local set of keys for the Google Auth. See [this guide](https://developers.google.com/identity/sign-in/web/sign-in) for guidance.
  - `REDIRECT_DOMAIN` is the full protocol and domain for your project. Ex `http://localhost:8080`, `https://rideboard-v2.cs.house`.

#### Running Program

1. Build Frontend. `cd src/frontend; npm run build`

2. Build server binary. `cargo build`

3. Run! `./target/debug/rideboard-v2`

### Frontend Development Tip

In order to develop the frontend without repeatedly recompiling the backend binary, the vite development server has been configured to proxy to `localhost:8080` for all API requests.

You can access the frontend and make changes at the port shown in the vite CLI. Please note that logging in will redirect you back to port 8080, so make sure to change it back when you log in.
