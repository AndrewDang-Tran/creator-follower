# creator-follower
Follow individual creators through RSS Feeds

#### Setup Development
```
cd cf-server; cargo run;
```

#### Setup Debian VM for docker containers
```
// Setup the
sudo apt-get update

sudo apt-get install \
  ca-certificates \
  curl \
  gnupg \
  lsb-release

sudo mkdir -p /etc/apt/keyrings

curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

```
