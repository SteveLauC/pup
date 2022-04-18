if [[ "$OSTYPE" == "linux"* ]];then
    curl -o pup https://raw.githubusercontent.com/stevelauc/pup/main/prebuilt-binary/pup-linux-amd64
elif [[ "$OSTYPE" == "darwin"* ]];then
    curl -o pup https://raw.githubusercontent.com/stevelauc/pup/main/prebuilt-binary/pup-macos-arm64
fi

# delete the previous version first
if ls /usr/local/bin|grep pup > /dev/null;then
    rm /usr/local/bin/pup
fi
mv pup /usr/local/bin
chmod +x /usr/local/bin

# Check if the installation is successful 
if ls /usr/local/bin|grep pup > /dev/null;then
    echo "pup is installed"
else
    echo "pup is not installed"
fi