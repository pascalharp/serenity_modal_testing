use dotenv::dotenv;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        id::GuildId,
        interactions::{
            message_component::{ActionRowComponent, ButtonStyle, InputTextStyle},
            Interaction, InteractionResponseType,
        },
        prelude::Ready,
    },
    Client,
};
use std::env;
use tracing_subscriber;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("modal")
                    .description("Trigger a modal from slash commands")
            })
        })
        .await;
        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content != "modal" {
            return;
        }

        msg.channel_id
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
                        d.custom_id("modal_comp_interaction");
                        d.title("Modals from Component Interaction");
                        d.components(|c| {
                            c.create_action_row(|ar| {
                                ar.create_input_text(|it| {
                                    it.style(InputTextStyle::Short)
                                        .custom_id("input_text_one")
                                        .required(true)
                                        .value("This is pre filled for you")
                                        .label("Short test")
                                })
                            });
                            c.create_action_row(|ar| {
                                ar.create_input_text(|it| {
                                    it.style(InputTextStyle::Paragraph)
                                        .custom_id("input_text_two")
                                        .required(false)
                                        .label("Loooooooooong text")
                                })
                            })
                        })
                    })
                })
                .await
                .unwrap();
            }
            Interaction::ApplicationCommand(aci) => {
                aci.create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::Modal);
                    r.interaction_response_data(|d| {
                        d.custom_id("modal_app_cmd");
                        d.title("Modals from Slash Commands");
                        d.components(|c| {
                            c.create_action_row(|ar| {
                                ar.create_input_text(|it| {
                                    it.style(InputTextStyle::Short)
                                        .custom_id("input_text_one")
                                        .required(false)
                                        .label("Short test")
                                })
                            });
                            c.create_action_row(|ar| {
                                ar.create_input_text(|it| {
                                    it.style(InputTextStyle::Paragraph)
                                        .custom_id("input_text_two")
                                        .required(true)
                                        .min_length(200)
                                        .placeholder("Please write an essay")
                                        .label("Loooooooooong text")
                                })
                            })
                        })
                    })
                })
                .await
                .unwrap();
            }
            Interaction::ModalSubmit(mci) => {
                dbg!(&mci);
                let short_text = match mci
                    .data
                    .components
                    .get(0)
                    .unwrap()
                    .components
                    .get(0)
                    .unwrap()
                {
                    ActionRowComponent::InputText(it) => it,
                    _ => return,
                };
                let long_text = match mci
                    .data
                    .components
                    .get(1)
                    .unwrap()
                    .components
                    .get(0)
                    .unwrap()
                {
                    ActionRowComponent::InputText(it) => it,
                    _ => return,
                };
                mci.create_interaction_response(ctx, |r| {
                    if mci.data.custom_id == "modal_app_cmd" {
                        r.kind(InteractionResponseType::ChannelMessageWithSource);
                    } else {
                        r.kind(InteractionResponseType::UpdateMessage);
                    }
                    r.interaction_response_data(|d| {
                        d.create_embed(|e| {
                            e.title(&short_text.value);
                            e.field(&long_text.custom_id, &long_text.value, false)
                        })
                    })
                })
                .await
                .unwrap();
            }
            _ => (),
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
