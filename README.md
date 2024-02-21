see https://blog.devgenius.io/creating-an-api-with-rust-clean-architecture-axum-and-surrealdb-2a95b1b72e0f
https://blog.devgenius.io/creating-an-api-with-rust-clean-architecture-cqrs-axum-and-surrealdb-part-2-99a48b2d10bc

https://codeculturepro.medium.com/implementing-google-login-in-a-node-js-application-b6fbd98ce5e  

https://github.com/wpcodevo/google-github-oauth2-rust/blob/master/src/handler.rs


## Docker 

```bash
$ docker-compose up -d # initialization scripts in ./containers/postgres/docker-entrypoint-initdb.d
$ docker exec -it url_shortener_postgres psql -U admin -d urlshotener
```