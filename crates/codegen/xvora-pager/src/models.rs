//! `xvora models` subcommand.

use anyhow::Result;
use tokio_util::sync::CancellationToken;
use xvora_shell::agent::config::Config as AgentConfig;
use xvora_shell::cli_models::{AuthStatus, list_models};

use crate::client_identity::{PAGER_CLIENT_TYPE, PAGER_CLIENT_VERSION};

pub async fn list_available_models(agent_config: &AgentConfig) -> Result<()> {
    match AuthStatus::resolve(agent_config) {
        AuthStatus::ApiKey => println!("You are using XAI_API_KEY."),
        AuthStatus::LoggedIn(host) => println!("You are logged in with {}.", host),
        AuthStatus::ModelCredentials(model) => {
            println!("Model '{model}' is using its own API key.");
        }
        AuthStatus::DeploymentKey => println!("You are authenticated via deployment key."),
        AuthStatus::NotAuthenticated => println!("You are not authenticated."),
    }
    println!();

    let cancel = CancellationToken::new();
    let spawned = crate::acp::spawn::spawn_grok_shell(agent_config.clone(), &cancel, None).await?;

    let state = list_models(&spawned.channel.tx, PAGER_CLIENT_TYPE, PAGER_CLIENT_VERSION).await?;

    println!("Default model: {}", state.current_model_id.0);
    println!();
    println!("Available models (provider · id):");
    // Sort by provider then id for a multi-provider-friendly list.
    let mut rows: Vec<_> = state.available_models.into_iter().collect();
    rows.sort_by(|a, b| {
        let pa = a
            .meta
            .as_ref()
            .and_then(|m| m.get("provider"))
            .and_then(|v| v.as_str())
            .unwrap_or("custom");
        let pb = b
            .meta
            .as_ref()
            .and_then(|m| m.get("provider"))
            .and_then(|v| v.as_str())
            .unwrap_or("custom");
        pa.cmp(pb).then_with(|| a.model_id.0.cmp(&b.model_id.0))
    });
    for m in rows {
        let provider = m
            .meta
            .as_ref()
            .and_then(|meta| meta.get("provider"))
            .and_then(|v| v.as_str())
            .unwrap_or("custom");
        let mark = if m.model_id == state.current_model_id {
            "*"
        } else {
            "-"
        };
        let default = if m.model_id == state.current_model_id {
            " (default)"
        } else {
            ""
        };
        println!("  {mark} {provider} · {}{default}", m.model_id.0);
    }

    cancel.cancel();
    Ok(())
}
