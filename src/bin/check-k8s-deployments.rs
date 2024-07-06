// use std::collections::HashMap;

use check_k8s::{cli::CliOpt, logging::configure_logging};
use clap::Parser;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tracing::{debug, info};

#[derive(Parser)]
struct DeploymentCliOpts {
    #[clap(flatten)]
    commonopts: CliOpt,

    #[clap(short = 'D', long)]
    /// Name of the deployment to check
    deployment: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = DeploymentCliOpts::parse();
    configure_logging(&opts.commonopts)?;

    let client = Client::try_default().await?;

    let deployments: Api<Deployment> = match opts.commonopts.namespace.clone() {
        Some(namespace) => Api::namespaced(client, &namespace),
        None => Api::all(client),
    };

    let deploymentlist = deployments.list(&ListParams::default()).await?;

    let mut bad_deployments = 0;
    let mut total_deployments = 0;

    for deployment in deploymentlist {
        let status = deployment
            .status
            .as_ref()
            .expect("Couldn't get deployment status!");

        let name = deployment
            .metadata
            .name
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or("Unknown".to_string());

        let available = status.available_replicas.unwrap_or(0);
        let replicas = status.replicas.unwrap_or(0);
        let unavailable = status.unavailable_replicas.unwrap_or(0);

        let ratio = available as f64 / replicas as f64;

        if opts.commonopts.debug {
            debug!(
                "{name}: available: {available}, replicas: {replicas}, unavailable: {unavailable}, ratio: {ratio}"
            );
        }

        if ratio < 1.0 {
            bad_deployments += 1;
        }
        total_deployments += 1;
    }

    // let bad = calculate_bad(&stats, &OK_PHASES);

    if bad_deployments > 0 {
        info!(
            "CRITICAL: {} deployments are not running out of {} total",
            bad_deployments, total_deployments
        );
        std::process::exit(2);
    } else if total_deployments == 0 {
        let namespace_string = match opts.commonopts.namespace {
            Some(ns) => format!(" in namespace '{}'", ns),
            None => "".to_string(),
        };
        info!("UNKNOWN: No deployments found{}!", namespace_string);
        std::process::exit(3);
    } else {
        info!(
            "OK: 100% of deployments are running ({}/{})",
            total_deployments, total_deployments
        );
    }
    Ok(())
}
