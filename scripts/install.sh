#!/bin/bash
set -euo pipefail

# Install upbit-daemon as a systemd service
SERVICE_NAME=upbit-daemon
INSTALL_DIR=/opt/${SERVICE_NAME}
USER=upbit

echo "Building ${SERVICE_NAME}..."
cargo build --release

echo "Installing ${SERVICE_NAME}..."

# Create installation directory
sudo mkdir -p ${INSTALL_DIR}

# Copy binary
sudo cp target/release/${SERVICE_NAME} ${INSTALL_DIR}/
sudo chmod +x ${INSTALL_DIR}/${SERVICE_NAME}

# Copy config
if [ -f config.yaml ]; then
    sudo cp config.yaml ${INSTALL_DIR}/
fi

# Create user if not exists
if ! id -u ${USER} &>/dev/null; then
    sudo useradd --system --no-create-home --shell /bin/false ${USER}
fi

# Set ownership
sudo chown -R ${USER}:${USER} ${INSTALL_DIR}

# Install systemd service
sudo cp -n systemd/${SERVICE_NAME}.service /etc/systemd/system/
sudo systemctl daemon-reload

echo ""
echo "Installation complete!"
echo ""
echo "Next steps:"
echo "1. Edit /etc/systemd/system/${SERVICE_NAME}.service with your credentials (Environment lines)"
echo "2. Run 'sudo systemctl daemon-reload'"
echo "3. Run 'sudo systemctl enable ${SERVICE_NAME}' to start on boot"
echo "4. Run 'sudo systemctl start ${SERVICE_NAME}' to start the service"
echo "5. Run 'sudo systemctl status ${SERVICE_NAME}' to check status"
