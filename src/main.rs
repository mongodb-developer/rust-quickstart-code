use bson::{doc, Bson};
use chrono::TimeZone;
use chrono::Utc;
use mongodb;
use serde::{Deserialize, Serialize};
use std::env;

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

    // Get the 'movies' collection from the 'sample_mflix' database:
    let movies = client.database("sample_mflix").collection("movies");

    let new_doc = doc! {
        "title": "Parasite",
        "year": 2020,
        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
        "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
    };
    println!("New Document: {}", new_doc);
    let insert_result = movies.insert_one(new_doc.clone(), None)?;
    println!("New document ID: {}", insert_result.inserted_id);

    // Look up one document:
    let movie = movies
        .find_one(
            doc! {
                "title": "Parasite"
            },
            None,
        )?
        .expect("Missing 'Parasite' document.");
    println!("Movie: {}", movie);

    // Update the document:
    let update_result = movies.update_one(
        doc! {
            "_id": &insert_result.inserted_id,
        },
        doc! {
            "$set": { "year": 2019 }
        },
        None,
    )?;
    println!("Updated {} documents", update_result.modified_count);

    // Look up the document again to confirm it's been updated:
    let movie = movies
        .find_one(
            doc! {
                "_id": &insert_result.inserted_id,
            },
            None,
        )?
        .expect("Missing 'Parasite' document.");
    println!("Updated Movie: {}", &movie);

    // Delete all documents for movies called "Parasite":
    let delete_result = movies.delete_many(
        doc! {
            "title": "Parasite"
        },
        None,
    )?;
    println!("Deleted {} documents", delete_result.deleted_count);

    // Working with Document is a bit horrible:
    if let Some(title) = new_doc.get("title").and_then(Bson::as_str) {
        println!("title: {}", title);
    } else {
        println!("no title found");
    }

    // We can use `serde` to create structs which can serialize & deserialize between BSON:
    #[derive(Serialize, Deserialize, Debug)]
    struct Movie {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<bson::oid::ObjectId>,
        title: String,
        year: i32,
    }

    // Insert a serialized struct into MongoDB:
    let captain_marvel = Movie {
        id: None,
        title: "Captain Marvel".to_owned(),
        year: 2019,
    };

    // Convert `captain_marvel` to a Bson instance:
    let serialized_movie = bson::to_bson(&captain_marvel)?;
    if let Bson::Document(document) = serialized_movie {
        let insert_result = movies.insert_one(document, None)?;
        if let Bson::ObjectId(inserted_id) = insert_result.inserted_id {
            let captain_marvel_id = inserted_id;
            println!("Captain Marvel document ID: {:?}", &captain_marvel_id);

            // Retrieve Captain Marvel from the database, into a Movie struct:
            // Read the document from the movies collection:
            let loaded_movie = movies
                .find_one(Some(doc! { "_id":  captain_marvel_id.clone() }), None)?
                .expect("Document not found");

            // Deserialize the document into a Movie instance
            let loaded_movie_struct: Movie = bson::from_bson(bson::Bson::Document(loaded_movie))?;
            println!("Movie loaded from collection: {:?}", loaded_movie_struct);

            // Delete Captain Marvel from MongoDB:
            movies.delete_one(doc! {"_id": Bson::ObjectId(captain_marvel_id)}, None)?;
            println!("Captain Marvel document deleted.");
        }
    } else {
        panic!("Could not convert the Movie struct into a BSON document!");
    }

    Ok(())
}
