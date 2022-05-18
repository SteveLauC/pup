set -e

# fetch the corresponding bianry
if [[ "$OSTYPE" == "linux"* ]];then
    if [ `uname -m` = "x86_64" ];then
        echo "fetching pup for you..."
        curl -o pup https://raw.githubusercontent.com/stevelauc/pup/main/prebuilt-binary/pup-linux-amd64
    elif [ `uname -p` = "arm"* ];then
        echo "Not supported yet";exit 1; 
    else
        echo "Not supported yet";exit 1; 
    fi
elif [[ "$OSTYPE" == "darwin"* ]];then
    echo "You need to build it from source";
    exit 1;
fi

# if `/usr/local/bin` does not exist, create it
if [ ! -d "/usr/local/bin" ];then
    sudo mkdir /usr/local/bin
fi

# delete the previous version first
if [ -f "/usr/local/bin/pup" ];then
    sudo rm /usr/local/bin/pup
fi

sudo mv pup /usr/local/bin
sudo chmod +x /usr/local/bin/pup

# Check if the installation is successful 
if [ -f "/usr/local/bin/pup" ];then
    echo "pup is installed ✔"
else
    echo "pup is NOT installed ✘"
fi
