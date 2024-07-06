use std::collections::HashMap;

use check_k8s::{calculate_bad, cli::CliOpt, logging::configure_logging};
use clap::Parser;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tracing::{debug, info};

// Valid states: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-phase>
const OK_PHASES: [&str; 2] = ["Running", "Succeeded"];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = CliOpt::parse();
    configure_logging(&opts)?;

    let client = Client::try_default().await?;

    let pods: Api<Pod> = match opts.namespace {
        Some(namespace) => Api::namespaced(client, &namespace),
        None => Api::all(client),
    };

    let podlist = pods.list(&ListParams::default()).await?;

    let mut stats: HashMap<String, usize> = HashMap::new();

    for pod in podlist {
        let phase = pod
            .status
            .as_ref()
            .expect("Couldn't get pod status!")
            .phase
            .as_ref()
            .expect("Couldn't get pod phase!");

        if !stats.contains_key(phase) {
            stats.insert(phase.clone(), 1);
        } else {
            let count = stats.get_mut(phase).unwrap();
            *count += 1;
        }

        if opts.debug {
            debug!(
                "{}",
                serde_json::to_string(&pod).expect("Failed to serialize pod")
            );
        }
    }

    let bad = calculate_bad(&stats, &OK_PHASES);

    if bad > 0 {
        info!("CRITICAL: {} pods are not running", bad);
        std::process::exit(2);
    } else {
        info!("OK: All pods are running {:?}", stats);
    }
    Ok(())
}
