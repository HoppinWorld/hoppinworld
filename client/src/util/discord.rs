use amethyst_extra::DiscordRichPresence;

pub fn init_discord_rich_presence() -> DiscordRichPresence {
    DiscordRichPresence::new(
        498979571933380609,
        "Main Menu".to_string(),
        Some("large_image".to_string()),
        Some("Hoppin World".to_string()),
        None,
        None,
    )
}
