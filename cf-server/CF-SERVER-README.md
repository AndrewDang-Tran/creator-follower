#### Setup Development
```
cargo run;
```

#### Setup Debian VM for docker containers
Pulled from [docker documentation](https://docs.docker.com/engine/install/debian/#install-using-the-repository)
```
apt-get update;
apt-get install \
  ca-certificates \
  curl \
  gnupg \
  lsb-release;

mkdir -p /etc/apt/keyrings;
curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg;
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null;

apt-get update;
apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin;

apt-get install git docker-compose;
mkdir workspace; cd workspace;
git pull https://github.com/AndrewDang-Tran/creator-follower.git;
cd creator-follower; docker-compose up -d --build;
```
