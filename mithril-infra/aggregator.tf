resource "null_resource" "mithril-aggregator" {

  # trigger a deployment of the aggregator whwen the
  # image_id is updated
  triggers = {
    image_id = var.image_id
  }

  connection {
    type        = "ssh"
    user        = "curry"
    private_key = var.private_key
    host        = google_compute_instance.mithril-aggregator-testnet.network_interface.0.access_config.0.nat_ip
  }

  provisioner "file" {
    source      = "docker-compose.yaml"
    destination = "/home/curry/docker-compose.yaml"
  }

  provisioner "file" {
    source      = "Dockerfile.cardano"
    destination = "/home/curry/Dockerfile.cardano"
  }
  provisioner "file" {
    source      = ".dockerignore"
    destination = "/home/curry/.dockerignore"
  }

  # traefik setup
  provisioner "remote-exec" {
    inline = [
      "mkdir -p /home/curry/traefik",
      "touch /home/curry/traefik/acme.json && chmod 600 /home/curry/traefik/acme.json"
    ]
  }

  provisioner "file" {
    source      = "traefik/"
    destination = "/home/curry/traefik"
  }

  # logs shipment to grafana cloud
  provisioner "file" {
    source      = "promtail-config.yml"
    destination = "/home/curry/promtail-config.yml"
  }

  # prometheus configuration file, used in the docker-compose file
  provisioner "file" {
    source      = "prometheus.yml"
    destination = "/home/curry/prometheus.yml"
  }

  provisioner "remote-exec" {
    inline = [
      "NETWORK=preview NETWORK_MAGIC=2 IMAGE_ID=${var.image_id} GOOGLE_APPLICATION_CREDENTIALS_JSON='${var.google_application_credentials_json}' CURRENT_UID=$(id -u) DOCKER_GID=$(getent group docker | cut -d: -f3)  docker-compose -f /home/curry/docker-compose.yaml --profile all up -d"
    ]
  }
}
