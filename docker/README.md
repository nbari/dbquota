For starting a mariadb container type `make` (docker needs to be up and running)

Use this for creating 100 databases:

    $ for i in {1..100};
    mycli --port 13306 -uroot -ptest -e "create database dbquota_$i" &

> $ brew update && brew install mycli or  $ pip install mycli

For loading the database with a dummy table:

    $ for i in {1..100};
    mycli --port 13306 -uroot -ptest dbquota_$i < test_db.sql &

Create a test user:

    GRANT ALL PRIVILEGES ON dbquota_1.* TO 'monkey'@'%' IDENTIFIED BY 'test';

To increase size of table you could use:

    INSERT INTO test(uuid) SELECT UUID() FROM test;
