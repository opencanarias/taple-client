mod common;
use std::time::Duration;

use common::*;
use futures::FutureExt;
use serial_test::serial;
use taple_core::ApiModuleInterface;
use taple_core::{identifier::Derivable, Event, SubjectData};
use ureq::Agent;

#[test]
#[serial]
fn init_node() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();
        let node = NodeBuilderAPI::new()
            .with_p2p_port(40069)
            .with_seed("40000".into())
            .run_with_api()
            .await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let result = utils::do_task_with_timeout(node.shutdown().boxed(), 1000).await;
        assert!(result.is_ok());
    });
}

#[test]
#[serial]
fn event_creation() {
    use taple_core::event_request::RequestData;

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        let node = NodeBuilderAPI::new()
            .with_p2p_port(40000)
            .with_seed("40000".into())
            .with_timeout(100)
            .with_pass_votation(1)
            .with_dev_mode(true)
            .with_http_port(3001)
            .run_with_api()
            .await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let node_two = NodeBuilderAPI::new()
            .with_p2p_port(40001)
            .with_seed("40001".into())
            .with_timeout(100)
            .with_pass_votation(1)
            .with_dev_mode(true)
            .with_http_port(3002)
            .add_access_point(
                "/ip4/127.0.0.1/tcp/40000/p2p/12D3KooWBGEMfdAeRHp5eZ1zTpyEeZyvJYoBrDo9WLEtjWZWnCwD"
                    .into(),
            )
            .run_with_api()
            .await;
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Governance is created
        let result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "Create": {
                        "governance_id": "",
                        "namespace": "",
                        "schema_id": "governance",
                        "payload": {
                            "Json": utils::governance_two()
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let governance_id = result.subject_id.unwrap();
        let governance_subj_data: SubjectData = ureq::get(&format!(
            "http://localhost:3001/api/subjects/{}",
            governance_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        assert_eq!(governance_id, governance_subj_data.subject_id.to_str());

        // A subject for governance is created
        let result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "Create": {
                        "governance_id": governance_id,
                        "namespace": "namespace1",
                        "schema_id": "prueba",
                        "payload": {
                            "Json": {
                                "a": "69"
                            }
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();
        let _subject_id = result.subject_id.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        // An event is created for the creation of a subject with the schema established in governance
        let result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "Create": {
                        "governance_id": governance_id,
                        "namespace": "namespace1",
                        "schema_id": "prueba",
                        "payload": {
                            "Json": {
                                "a": "69"
                            }
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();
        let subject_id = result.subject_id.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;

        // An event is created for the subject change
        let _result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "State": {
                        "subject_id": subject_id,
                        "payload": {
                            "Json": {
                                "a": "70"
                            }
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();

        // The event that has been created is checked
        tokio::time::sleep(Duration::from_secs(1)).await;
        let result2: Event = ureq::get(&format!(
            "http://localhost:3002/api/subjects/{}/events/1",
            subject_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let result3: Event = ureq::get(&format!(
            "http://localhost:3001/api/subjects/{}/events/1",
            subject_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        assert_eq!(result2, result3);
        let result = node_two
            .get_signatures(subject_id.clone(), 1, None, None)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);

        // An event is created for the subject change
        let _result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "State": {
                        "subject_id": subject_id,
                        "payload": {
                            "Json": {"a": "71"}
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let result2: Event = ureq::get(&format!(
            "http://localhost:3002/api/subjects/{}/events/2",
            subject_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let result3: Event = ureq::get(&format!(
            "http://localhost:3002/api/subjects/{}/events/2",
            subject_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        assert_eq!(result2, result3);
        let result = node_two
            .get_signatures(subject_id.clone(), 2, None, None)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
        tokio::time::sleep(Duration::from_millis(100)).await;
        let result = node.shutdown().await;
        assert!(result.is_ok());
        let result = node_two.shutdown().await;
        assert!(result.is_ok());
    });
}

#[test]
#[serial]
fn add_new_member_to_governance() {
    use taple_core::event_request::RequestData;

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        let _node = NodeBuilderAPI::new()
            .with_p2p_port(40000)
            .with_seed("40000".into())
            // .with_pass_votation(1)
            // .with_dev_mode(true)
            .with_timeout(100)
            .with_http_port(3001)
            .run_with_api()
            .await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let _node_two = NodeBuilderAPI::new()
            .with_p2p_port(40001)
            .with_seed("40001".into())
            .with_timeout(100)
            // .with_pass_votation(1)
            // .with_dev_mode(true)
            .with_http_port(3002)
            .add_access_point(
                "/ip4/127.0.0.1/tcp/40000/p2p/12D3KooWBGEMfdAeRHp5eZ1zTpyEeZyvJYoBrDo9WLEtjWZWnCwD"
                    .into(),
            )
            .run_with_api()
            .await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "Create": {
                        "governance_id": "",
                        "namespace": "",
                        "schema_id": "governance",
                        "payload": {
                            "Json": utils::governance_one()
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let governance_id = result.subject_id.unwrap();
        let _governance_subj_data: SubjectData = ureq::get(&format!(
            "http://localhost:3001/api/governances/{}",
            governance_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        let result: RequestData = ureq::post(&format!("http://localhost:3001/api/requests"))
            .set("X-API-KEY", "apikeyexamplevalue123")
            .send_json(serde_json::json!({
                "request": {
                    "State": {
                        "subject_id": governance_id,
                        "payload": {
                            "Json": utils::governance_two()
                        }
                    }
                }
            }))
            .unwrap()
            .into_json()
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let request_id = result.request_id;
        let _: () = ureq::put(&format!(
            "http://localhost:3001/api/approvals/{}",
            request_id
        ))
        .set("X-API-KEY", "apikeyexamplevalue123")
        .send_json(serde_json::json!({"approvalType": "Accept"}))
        .unwrap()
        .into_json()
        .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let _governance_subj_data: SubjectData = ureq::get(&format!(
            "http://localhost:3002/api/subjects/{}",
            governance_id
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    });
}
