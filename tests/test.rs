use std::collections::HashMap;

use nagios_k8s::calculate_bad;

#[test]
fn test_calculate_bad() {
    let stats = HashMap::from([("Foo".to_string(), 1), ("Bar".to_string(), 2)]);

    let ok_status = ["Foo", "Bar"];
    assert_eq!(calculate_bad(&stats, &ok_status), 0);
    let ok_status = ["Foo"];
    assert_eq!(calculate_bad(&stats, &ok_status), 2);
    let ok_status = [];
    assert_eq!(calculate_bad(&stats, &ok_status), 3);
}

#[test]
fn test_log_setup_debug() {
    assert!(
        nagios_k8s::logging::configure_logging(&nagios_k8s::cli::CliOpt {
            debug: true,
            ..nagios_k8s::cli::CliOpt::default()
        })
        .is_ok()
    );
}
