use bson::{doc, Bson};
use mongodb;
use std::env;
use chrono::Utc;
use chrono::TimeZone;

fn main() -> Result<(), mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    let client = mongodb::Client::with_uri_str(client_uri.as_ref())?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None)? {
        println!("- {}", name);
    }

    // Get our 'movies' collection:
    let movies = client.database("sample_mflix").collection("movies");

    let new_doc = doc! {
        "title": "Parasite",
        "year": 2020,
        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
        "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
    };
    println!("New Document: {}", new_doc);

    let insert_result = movies.insert_one(new_doc, None)?;
    println!("New document ID: {}", insert_result.inserted_id);

    // Look up one document:
    let movie: bson::Document = movies.find_one(doc!{
        "title": "Parasite"
    }, None)?.expect("Missing 'Parasite' document.");
    println!("Movie: {}", Bson::from(movie));

    // // Look up one document:
    // match movies.find_one(doc!{
    //     "year": "The Great Train Robbery"
    // }, None).expect("Error retrieving 'The Great Train Robbery'") {
    //     None => println!("Missing 'The Great Train Robbery' document."),
    //     Some(movie) => {
    //         println!("Movie: {}", movie.get("title").unwrap());
    //         println!("Year: {}", movie.get("year").unwrap());
    //     }
    // };

    // movies.update_one(doc!{"_id": insert_result.inserted_id}, update: impl Into<UpdateModifications>, None)

    Ok(())
}
