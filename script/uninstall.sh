set -e

pup --delete-token;
if [ -f "/usr/local/bin/pup" ];then
    echo "deleting binary..."
    sudo rm /usr/local/bin/pup
fi

if ls ~/.config|grep pup;then
    echo "deleting configuration file..."
    rm -r ~/.config/pup
fi
echo "pup is deleted ✔"