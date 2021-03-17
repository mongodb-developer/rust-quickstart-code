use mongodb::bson::{self, doc};
use mongodb::options::{ClientOptions, ResolverConfig};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fmt;
use tokio;
use tokio::stream::StreamExt;

#[derive(Deserialize)]
struct MovieSummary {
    title: String,
    cast: Vec<String>,
    year: i32,
    #[serde(default, alias = "related_comments")]
    comments: Vec<Comment>,
}

impl fmt::Display for MovieSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.title,
            self.cast.get(0).unwrap_or(&"- no cast -".to_owned()),
            self.year
        )
    }
}

#[derive(Debug, Deserialize)]
struct Comment {
    email: String,
    name: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct YearSummary {
    //#[serde(alias = "_id")]
    _id: i32,
    #[serde(default)]
    movie_count: i64,
    #[serde(default)]
    movie_titles: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = mongodb::Client::with_options(options)?;

    // Get the 'movies' collection from the 'sample_mflix' database:
    let movies = client.database("sample_mflix").collection("movies");

    // -----------------------------------------------------------------------
    println!("A Star is Born, Sorted by Year:");
    let pipeline = vec![
        doc! {
            // filter on movie title:
            "$match": {
                "title": "A Star Is Born"
            }
        },
        doc! {
            // sort by year, ascending:
            "$sort": {
                "year": 1
            }
        },
    ];

    // Look up "A Star is Born" in ascending year order:
    let mut results = movies.aggregate(pipeline, None).await?;
    // Loop through the results, convert them to a MovieSummary, and then print out.
    while let Some(result) = results.next().await {
        let doc: MovieSummary = bson::from_document(result?)?;
        println!("* {}", doc);
    }

    // -----------------------------------------------------------------------
    println!("Most Recent Production:");

    // Match title = "A Star Is Born":
    let stage_match_title = doc! {
       "$match": {
             "title": "A Star Is Born"
       }
    };

    // Sort by year, descending:
    let stage_sort_year_descending = doc! {
        "$sort": {
            "year": -1
        }
    };

    // Limit to 1 document:
    let stage_limit_1 = doc! { "$limit": 1 };

    let pipeline = vec![stage_match_title, stage_sort_year_descending, stage_limit_1];

    let mut results = movies.aggregate(pipeline, None).await?;
    // Loop through the results (there will only be one), and print the year:
    while let Some(result) = results.next().await {
        let doc: MovieSummary = bson::from_document(result?)?;
        println!("* {}", doc);
    }

    // -----------------------------------------------------------------------
    println!("Movie Comments");

    // Look up related documents in the 'comments' collection:
    let stage_lookup_comments = doc! {
       "$lookup": {
             "from": "comments",
             "localField": "_id",
             "foreignField": "movie_id",
             "as": "related_comments",
       }
    };

    // Limit to the first 5 documents:
    let stage_limit_5 = doc! { "$limit": 5 };

    // Calculate the number of comments for each movie:
    let stage_add_comment_count = doc! {
       "$addFields": {
             "comment_count": {
                "$size": "$related_comments"
             }
       }
    };

    // Match movie documents with more than 2 comments:
    let stage_match_with_comments = doc! {
       "$match": {
             "comment_count": {
                "$gt": 2
             }
       }
    };

    let pipeline = vec![
        stage_lookup_comments,
        stage_add_comment_count,
        stage_match_with_comments,
        stage_limit_5,
    ];

    let mut results = movies.aggregate(pipeline, None).await?;
    // Loop through the results and print a summary and the comments:
    while let Some(result) = results.next().await {
        let doc: MovieSummary = bson::from_document(result?)?;
        println!("* {}", doc);
        if doc.comments.len() > 0 {
            // Print a max of 5 comments per movie:
            for comment in doc.comments.iter().take(5) {
                println!(
                    "  - {} <{}>: {}",
                    comment.name,
                    comment.email,
                    comment.text.chars().take(60).collect::<String>(),
                );
            }
        } else {
            println!("  - No comments");
        }
    }

    // -----------------------------------------------------------------------
    println!("Grouping Documents:");

    // Some movies have "year" values ending with 'è'.
    // This stage will filter them out:
    let stage_filter_valid_years = doc! {
        "$match": {
            "year": {
                "$type": "number",
            }
        }
    };

    /*
     * Group movies by year, producing 'year-summary' documents that look like:
     * {
     *     '_id': 1917,
     * }
     */
    let stage_group_year = doc! {
       "$group": {
            "_id": "$year",
            // Count the number of movies in the group:
            "movie_count": { "$sum": 1 },
            "movie_titles": { "$push": "$title" },
       }
    };

    let stage_sort_year_ascending = doc! {
      "$sort": {"_id": 1}
    };

    let pipeline = vec![
        stage_filter_valid_years,
        stage_group_year,
        stage_sort_year_ascending,
    ];

    // Loop through the 'year-summary' documents:
    let mut results = movies.aggregate(pipeline, None).await?;
    // Loop through the results and print a summary and the comments:
    while let Some(result) = results.next().await {
        let doc: YearSummary = bson::from_document(result?)?;
        println!("* {:?}", doc);
    }

    Ok(())
}
