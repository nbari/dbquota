For starting a mariadb container type `make` (docker needs to be up and running)

Use this for creating 100 databases:

    $ for i in {1..100};
    mycli --port 13306 -uroot -ptest -e "create database dbquota_$i" &

> $ brew update && brew install mycli or  $ pip install mycli

For loading the database with a dummy table:

    $ for i in {1..100};
    mycli --port 13306 -uroot -ptest dbquota_$i < test_db.sql &