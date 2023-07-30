
resource "null_resource" "mithril_monitoring" {
  depends_on = [
    null_resource.mithril_network,
    null_resource.mithril_reverse_proxy,
    null_resource.mithril_aggregator,
    null_resource.mithril_signer
  ]

  triggers = {
    image_id                 = var.mithril_image_id,
    vm_instance              = google_compute_instance.vm_instance.id,
    prometheus_auth_username = var.prometheus_auth_username,
    prometheus_auth_password = var.prometheus_auth_password,
    loki_auth_username       = var.loki_auth_username,
    loki_auth_password       = var.loki_auth_password
  }

  connection {
    type        = "ssh"
    user        = "curry"
    private_key = local.google_service_account_private_key
    host        = google_compute_address.mithril-external-address.address
  }

  provisioner "remote-exec" {
    inline = [
      "mkdir -p /home/curry/data/monitoring/prometheus",
      "mkdir -p /home/curry/data/monitoring/loki",
      <<-EOT
# Setup prometheus targets configuration for Cardano nodes
CARDANO_NODES=$(docker ps --format='{{.Names}}:12798,' | grep "cardano-node" | sort | tr -d '\n\t\r ' | sed 's/.$//')
cat /home/curry/docker/prometheus/cardano.json | jq --arg CARDANO_NODES "$CARDANO_NODES" '. += [{
    "labels": {
        "job": "cardano-nodes"
    },
    "targets": $CARDANO_NODES
}]' | jq '. | map(try(.targets |= split(",")) // .)' > /home/curry/docker/prometheus/cardano.json.new
rm -f /home/curry/docker/prometheus/cardano.json
mv /home/curry/docker/prometheus/cardano.json.new docker/prometheus/cardano.json
EOT
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "export LOGGING_DRIVER='${var.mithril_container_logging_driver}'",
      "export PROMETHEUS_HOST=${local.prometheus_host}",
      "export PROMETHEUS_AUTH_USER_PASSWORD=$(htpasswd -nb ${var.prometheus_auth_username} ${var.prometheus_auth_password})",
      "export LOKI_HOST=${local.loki_host}",
      "export LOKI_AUTH_USER_PASSWORD=$(htpasswd -nb ${var.loki_auth_username} ${var.loki_auth_password})",
      "export CURRENT_UID=$(id -u)",
      "docker compose -f /home/curry/docker/docker-compose-monitoring.yaml --profile all up -d",
    ]
  }
}
