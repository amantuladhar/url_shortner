# URL Shortner


# Tools

## sqlx
- Mirgration Ref Link : https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query

## Install sqlx cli
> cargo install sqlx-cli

## Create drop database
> sqlx database create
> sqlx database drop

## Create migration
> sqlx migrate add <name>
> sqlx migrate add -r <name> # for reversable migration

## Revert migration
> sqlx migrate revert

## Run migration
> sqlx migrate run

## Run migration as part of app startup
> sqlx::migrate!().run(&pool).await?;

## Offline representation of database
> cargo sqlx prepare
- Need "offline" feature of sqlx