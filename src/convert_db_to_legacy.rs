use crate::parse::{Channel, ChannelMessages, Data, Message, Meta, Server, User, DHT};
use sqlx::{Executor, Pool, Row, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

async fn connect_database(input_path: &Path) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !input_path.extension().is_some_and(|ext| ext.eq("dht")) {
        panic!("The input file must end with .dht extension.")
    }

    let original_path = Path::new(input_path);
    let prefix = Path::new("sqlite:");

    let prefixed_path = fs::canonicalize(prefix.join(original_path))?;
    SqlitePool::connect(prefixed_path.to_str().unwrap()).await
}

pub async fn convert_database_to_dht(input_path: &Path) -> Result<DHT, sqlx::Error> {
    let pool = connect_database(input_path).await?;

    println!("Querying users...");
    let users = query_users(&pool).await?;
    println!("Querying channels and servers...");
    let (servers, channels) = query_servers(&pool).await?;
    println!("Querying messages...");
    let channel_messages = query_channels_messages(&pool).await?;
    let user_index = users.keys().map(|id| id.to_string()).collect();

    Ok(DHT {
        meta: Meta {
            users,
            user_index,
            servers,
            channels,
        },
        data: Data { channel_messages },
    })
}

async fn query_users(pool: &SqlitePool) -> Result<HashMap<String, User>, sqlx::Error> {
    let mut users = HashMap::new();
    let rows = pool
        .fetch_all("SELECT id, name, discriminator AS tag, avatar_url AS avatar FROM users")
        .await?;

    for row in rows {
        let user = User {
            name: row.get("name"),
            tag: row.get("tag"),
            avatar: row.get("avatar"),
        };

        let id: i64 = row.get("id");
        users.insert(id.to_string(), user);
    }

    Ok(users)
}

async fn query_servers(
    pool: &SqlitePool,
) -> Result<(Vec<Server>, HashMap<String, Channel>), sqlx::Error> {
    let mut channels = HashMap::new();
    let mut servers = Vec::new();
    let rows = pool
        .fetch_all("SELECT channels.id as channel_id, servers.id as server, channels.name, type as channel_type FROM servers JOIN channels ON channels.server = servers.id")
        .await?;

    for i in 0..rows.len() {
        let row = rows.get(i).unwrap();

        servers.insert(
            i,
            Server {
                name: row.get("name"),
                channel_type: row.get("channel_type"),
            },
        );

        let channel = Channel {
            server: i as u64,
            name: row.get("name"),
        };

        let id: i64 = row.get("channel_id");
        channels.insert(id.to_string(), channel);
    }

    Ok((servers, channels))
}

async fn query_messages(
    pool: &SqlitePool,
    channel_id: &str,
) -> Result<HashMap<String, Message>, sqlx::Error> {
    let mut messages = HashMap::new();
    let query = format!("SELECT message_id, sender_id as user_index, timestamp, text as message_content FROM messages WHERE channel_id = {channel_id}");
    let rows = pool.fetch_all(query.as_str()).await?;

    for row in rows {
        let message = Message {
            user_index: row.get("user_index"),
            timestamp: row.get("timestamp"),
            message_content: row.try_get("message_content").ok(),
        };

        let id: i64 = row.get("message_id");
        messages.insert(id.to_string(), message);
    }

    Ok(messages)
}

async fn query_channels_messages(
    pool: &SqlitePool,
) -> Result<HashMap<String, ChannelMessages>, sqlx::Error> {
    let mut channels = HashMap::new();
    let rows = pool
        .fetch_all("SELECT id, server, name FROM channels")
        .await?;

    for row in rows {
        let id: i64 = row.get("id");
        let channel_id: String = id.to_string();
        // Fetch messages for this channel
        let messages = query_messages(pool, &channel_id).await?;

        channels.insert(channel_id.clone(), messages);
    }

    Ok(channels)
}
