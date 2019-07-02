For starting a mariadb container type `make` (docker needs to be up and running)

Use this for creating 1000 databases:

    $ for i in {1..1000};
    mycli --port 13306 -uroot -ptest -e "create database dbquota_$i" &

> $ brew update && brew install mycli or  $ pip install mycli
