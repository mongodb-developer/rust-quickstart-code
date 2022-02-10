use chrono::{TimeZone, Utc};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use mongodb::bson::{Bson, Document, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
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
    let insert_result = movies.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    // Look up one document:
    let movie: Document = movies
        .find_one(
            doc! {
                "title": "Parasite"
            },
            None,
        )
        .await?
        .expect("Missing 'Parasite' document.");
    println!("Movie: {}", movie);
    let title = movie.get_str("title")?;
    // -> "Parasite"
    println!("Movie Title: {}", title);

    let movie_json: serde_json::Value = Bson::from(movie.clone()).into();
    println!("JSON: {}", movie_json);

    // Update the document:
    let update_result = movies
        .update_one(
            doc! {
		"_id": &movie.get("_id")
            },
            doc! {
                "$set": { "year": 2019 }
            },
            None,
        )
        .await?;
    println!("Updated {} documents", update_result.modified_count);

    // Look up the document again to confirm it's been updated:
    let movie = movies
        .find_one(
            doc! {
		"_id": &movie.get("_id")
            },
            None,
        )
        .await?
        .expect("Missing 'Parasite' document.");
    println!("Updated Movie: {}", &movie);

    // Delete all documents for movies called "Parasite":
    let delete_result = movies
        .delete_many(
            doc! {
                "title": "Parasite"
            },
            None,
        )
        .await?;
    println!("Deleted {} documents", delete_result.deleted_count);

    // Working with Document is a bit horrible:
    if let Ok(title) = new_doc.get_str("title") {
        println!("title: {}", title);
    } else {
        println!("no title found");
    }

    // We can use `serde` to create structs which can serialize & deserialize between BSON:
    #[derive(Serialize, Deserialize, Debug)]
    struct Movie {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        title: String,
        year: i32,
        plot: String,
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        released: chrono::DateTime<Utc>,
    }

    // Initialize struct to be inserted:
    let captain_marvel = Movie {
        id: None,
        title: "Captain Marvel".to_owned(),
        year: 2019,
        plot: "Amidst a mission, Vers, a Kree warrior, gets separated from her team and is stranded on Earth. However, her life takes an unusual turn after she teams up with Fury, a S.H.I.E.L.D. agent.".to_owned(),
        released: Utc.ymd(2019,3,8).and_hms(0,0,0)
    };

    // Convert `captain_marvel` to a Bson instance:
    let serialized_movie = bson::to_bson(&captain_marvel)?;
    let document = serialized_movie.as_document().unwrap();

    // Insert into the collection and extract the inserted_id value:
    let insert_result = movies.insert_one(document.to_owned(), None).await?;
    let captain_marvel_id = insert_result
        .inserted_id
        .as_object_id()
        .expect("Retrieved _id should have been of type ObjectId");
    println!("Captain Marvel document ID: {:?}", captain_marvel_id);

    // Retrieve Captain Marvel from the database, into a Movie struct:
    // Read the document from the movies collection:
    let loaded_movie = movies
        .find_one(Some(doc! { "_id":  captain_marvel_id.clone() }), None)
        .await?
        .expect("Document not found");

    // Deserialize the document into a Movie instance
    let loaded_movie_struct: Movie = bson::from_bson(loaded_movie.into())?;
    println!("Movie loaded from collection: {:?}", loaded_movie_struct);

    // Delete Captain Marvel from MongoDB:
    movies
        .delete_one(doc! {"_id": captain_marvel_id.to_owned()}, None)
        .await?;
    println!("Captain Marvel document deleted.");

    Ok(())
}
