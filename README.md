# Setup

1. Create a new user on the server

       sudo useradd -m -s /bin/bash gargoyle

1. Log in as the user

        sudo -iu gargoyle

1. Ensure you have the nightly rust toolchain installed and selected

        rustup default nightly

1. Clone this repository into /opt/gargoyle

        sudo mkdir -p /opt/gargoyle
        chown gargoyle:gargoyle /opt/gargoyle
        chmod 700 /opt/gargoyle
        git clone https://github.com/Mordinel/my-website-monitor.git /opt/gargoyle
        cd /opt/gargoyle

1. Build the example of your choice

        cargo b --release
    
1. If needed, copy the environment file to the same location as the service file defines and
   modify its contents

        cp my-monitor.env .env
        chmod 600 .env

1. Modify the service file to your liking and copy it into the systemd service location

        exit # to stop being the gargoyle user
        sudo cp /opt/gargoyle/my-monitor.service /etc/systemd/system/
        sudo systemctl daemon-reload

1. Enable the service in systemd

        sudo systemctl enable --now my-monitor.service
