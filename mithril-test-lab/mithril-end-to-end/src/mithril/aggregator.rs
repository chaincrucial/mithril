use crate::devnet::BftNode;
use crate::utils::MithrilCommand;
use crate::DEVNET_MAGIC_ID;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::process::Child;

#[derive(Debug)]
pub struct Aggregator {
    server_port: u64,
    db_directory: PathBuf,
    command: MithrilCommand,
    process: Option<Child>,
}

impl Aggregator {
    pub fn new(
        server_port: u64,
        bft_node: &BftNode,
        cardano_cli_path: &Path,
        work_dir: &Path,
        bin_dir: &Path,
    ) -> Result<Self, String> {
        let port = server_port.to_string();
        let magic_id = DEVNET_MAGIC_ID.to_string();
        let env = HashMap::from([
            ("NETWORK", "devnet"),
            ("PROTOCOL_PARAMETERS__K", "5"),
            ("PROTOCOL_PARAMETERS__M", "100"),
            ("PROTOCOL_PARAMETERS__PHI_F", "0.65"),
            ("RUN_INTERVAL", "600"),
            ("URL_SNAPSHOT_MANIFEST", ""),
            ("SNAPSHOT_STORE_TYPE", "local"),
            ("SNAPSHOT_UPLOADER_TYPE", "local"),
            ("NETWORK_MAGIC", &magic_id),
            ("DATA_STORES_DIRECTORY", "./stores/aggregator"),
            (
                "CARDANO_NODE_SOCKET_PATH",
                bft_node.socket_path.to_str().unwrap(),
            ),
            ("CARDANO_CLI_PATH", cardano_cli_path.to_str().unwrap()),
            ("GENESIS_VERIFICATION_KEY", "5b33322c3235332c3138362c3230312c3137372c31312c3131372c3133352c3138372c3136372c3138312c3138382c32322c35392c3230362c3130352c3233312c3135302c3231352c33302c37382c3231322c37362c31362c3235322c3138302c37322c3133342c3133372c3234372c3136312c36385d"),
        ]);
        let args = vec![
            "--db-directory",
            bft_node.db_path.to_str().unwrap(),
            "--server-port",
            &port,
            "-vvv",
        ];

        let command = MithrilCommand::new("mithril-aggregator", work_dir, bin_dir, env, &args)?;

        Ok(Self {
            server_port,
            db_directory: bft_node.db_path.clone(),
            command,
            process: None,
        })
    }

    pub fn endpoint(&self) -> String {
        format!("http://localhost:{}/aggregator", &self.server_port)
    }

    pub fn db_directory(&self) -> &Path {
        &self.db_directory
    }

    pub fn serve(&mut self) -> Result<(), String> {
        self.process = Some(self.command.start(&["serve".to_string()])?);
        Ok(())
    }

    pub async fn bootstrap_genesis(&mut self) -> Result<(), String> {
        let mut child = self
            .command
            .start(&["genesis".to_string(), "bootstrap".to_string()])?;

        match child.wait().await {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(match status.code() {
                        Some(c) => format!(
                            "`mithril-aggregator genesis bootstrap` exited with code: {}",
                            c
                        ),
                        None => {
                            "`mithril-aggregator genesis bootstrap` was terminated with a signal"
                                .to_string()
                        }
                    })
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        if let Some(process) = self.process.as_mut() {
            process
                .kill()
                .await
                .map_err(|e| format!("Could not kill aggregator: {:?}", e))?;
        }
        Ok(())
    }

    pub async fn tail_logs(&self, number_of_line: u64) -> Result<(), String> {
        self.command.tail_logs(None, number_of_line).await
    }
}
