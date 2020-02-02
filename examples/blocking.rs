extern crate futures;
extern crate reql;
extern crate reql_types;

use futures::Stream;
use reql::{Client, Config, Document, Run};
use reql_types::ServerStatus;

fn main() -> reql::Result<()> {
    // Create a new ReQL client
    let r = Client::new();

    // Create a connection pool
    let conn = r.connect(Config::default())?;

    // Run the query
    let stati = r
        .db("rethinkdb")
        .table("server_status")
        .run::<ServerStatus>(conn)?;

    // Process the results
    match stati.wait().next().unwrap() {
        // The server returned the response we were expecting
        Ok(Some(Document::Expected(status))) => {
            println!("{:?}", status);
        }
        // We got a response alright, but it wasn't the one we were
        // expecting plus it's not an error either, otherwise it would
        // have been returned as such (This simply means that the response
        // we got couldn't be serialised into the type we were expecting)
        Ok(Some(Document::Unexpected(status))) => {
            println!("unexpected response from server: {:?}", status);
        }
        // This is impossible in this particular example since there
        // needs to be at least one server available to give this
        // response otherwise we would have run into an error for
        // failing to connect
        Ok(None) => {
            println!("got no documents in the database");
        }
        // Oops! We ran into an error
        Err(error) => {
            println!("error: {}", error);
        }
    }

    Ok(())
}
