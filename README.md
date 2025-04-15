# rust-bridge-scorecard-api

This is the backend API for the bridge scorecard app.  Instead of keeping track of your scores at a bridge game on paper, use the app instead.  Note that I suspect actually using electronic scorekeeping would likely be found against ACBL rules, as it could allow a way for collusion to happen.

The real goal with this app was to teach myself Rust and Axum.

## Database considerations
The current main branch of this app uses MongoDB.  I started the initial version of this app in Node/Express, with MongoDB, again, for the learning experience.  Over the course of developing this app, I have come to think that using a schemaless database with a (*very*) strongly typed language like Rust is a really bad idea.  I can make the backend crash on simple listing calls just by sticking an extra key/value pair on one record.  That's right, you can break your app just by changing the underlying data in 1 record, not even the code.  Maybe there is a crate out there to mitigate this, or I could hack together a layer to stop that, but, that's a lot of work.  And, as someone said, "just because your database is schemaless, that doesn't make your data schemaless, and that schema price will be paid somewhere."

Because of this, I am migrating to Postgres.

