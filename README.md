# headless-backend-experiment
## Overview
This project is experiment of headless backend.
I'm interested in development for almost every platform like web, desktop, cli, tui, mobile.
So this is my first step of it.

I have no plan what I'm gonna develop.
So at first, I just build simple api one. And as another project, I'm gonna develop interface like GUI app.

## features
- [x] github login
- [ ] user
- [ ] profile
- [ ] platform connection

## How to run

Two ways to start the development environment.

### devcontainer (recommended)

**Requirements**

- [Docker](https://docs.docker.com/engine/install/)
- SSH agent running with your key added (`ssh-add ~/.ssh/id_ed25519`)

**VS Code**

1. Install the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
2. Copy `.env.example` to `.env` and fill in the values
3. Open the folder in VS Code and run `Dev Containers: Reopen in Container`

**devcontainer CLI**

```sh
cp .env.example .env
# fill in GITHUB_CLIENT_ID and GITHUB_CLIENT_SECRET in .env

devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . cargo watch -x run
```

Ports 3000 (API) and 5432 (PostgreSQL) are forwarded to the host automatically.

### local with Nix

**Requirements**

- [Nix](https://nixos.org/download/) with flakes enabled
- [direnv](https://direnv.net/) with nix-direnv

**Steps**

```sh
cp .env.example .env
# fill in GITHUB_CLIENT_ID and GITHUB_CLIENT_SECRET in .env

direnv allow   # activates nix dev shell automatically on cd
cargo watch -x run
```

You also need a PostgreSQL instance running locally and `DATABASE_URL` set in `.env`.

## Lisence
I forgot adding lisence.
I'm gonna add it before if I didn't forget.
