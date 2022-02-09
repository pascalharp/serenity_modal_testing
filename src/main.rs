use std::env;
use tracing_subscriber;
use dotenv::dotenv;
use serenity::{Client, async_trait, client::{EventHandler, Context}, model::{channel::Message, interactions::{Interaction, message_component::{ButtonStyle, InputTextStyle}, InteractionResponseType}}};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content != "modal" {
            return;
        }

        msg
            .channel_id
            .send_message(&ctx, |m| {
                m.content("Press the button for modal =D");
                m.components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|button| {
                            button
                                .style(ButtonStyle::Primary)
                                .label("Press for Modal")
                                .custom_id("modal_init")
                        })
                    })
                })
            })
            .await
            .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::MessageComponent(mci) => {
                mci.create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::Modal);
                    r.interaction_response_data(|d| {
                        d.custom_id("modal");
                        d.title("Modals POG");
                        d.components(|c| {
                            c.create_action_row(|ar| {
                                ar.create_input_text(|it| {
                                    it
                                        .style(InputTextStyle::Short)
                                        .custom_id("input_text_one")
                                        .label("Short test")
                                })
                            });
                            c.create_action_row(|ar| {
                                ar.create_input_text(|it| {
                                    it
                                        .style(InputTextStyle::Paragraph)
                                        .custom_id("input_text_two")
                                        .label("Loooooooooong text")
                                })
                            })
                        })
                    })
                })
                    .await
                    .unwrap();
            },
            Interaction::ModalSubmit(mci) => {
                dbg!(mci);
            },
            _ => ()
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id. It is needed for components
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
