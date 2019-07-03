use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    MySQL(mysql::Error),
    RowError(mysql::FromRowError),
    RowExpected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MySQL(ref err) => err.fmt(f),
            Error::RowError(ref err) => err.fmt(f),
            Error::RowExpected => write!(f, "row expected"),
        }
    }
}

impl error::Error for Error {}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Self {
        Error::MySQL(err)
    }
}

impl From<mysql::FromRowError> for Error {
    fn from(err: mysql::FromRowError) -> Self {
        Error::RowError(err)
    }
}

pub struct Queries<'a> {
    pool: &'a mysql::Pool,
}

pub fn new(pool: &mysql::Pool) -> Queries {
    return Queries { pool: pool };
}

impl<'a> Queries<'a> {
    pub fn update_db_size(&self) -> Result<(), Error> {
        // create table
        self.pool.prep_exec(
            r#"CREATE TABLE IF NOT EXISTS quotas (
        name VARCHAR(64) NOT NULL,
        bytes BIGINT UNSIGNED NOT NULL,
        mbytes FLOAT UNSIGNED NOT NULL,
        quota BIGINT UNSIGNED,
        enabled TINYINT(1) DEFAULT 0,
        cdate timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        PRIMARY KEY(name))"#,
            (),
        )?;

        // update table with db size
        // optimize table <tbname> claims disk space
        let mut tr = self.pool.start_transaction(true, None, None)?;
        tr.prep_exec(
            r#"INSERT INTO quotas (name, bytes, mbytes)
            SELECT t.name, t.bytes, t.mbytes
            FROM
            (SELECT
	        table_schema 'name',
	        SUM(data_length + index_length) 'bytes',
	        ROUND(SUM(data_length + index_length) / 1024 / 1024, 2) 'mbytes'
            FROM information_schema.tables
            GROUP BY table_schema) as t
            ON DUPLICATE KEY UPDATE bytes=t.bytes, mbytes=t.mbytes"#,
            (),
        )?;
        tr.commit()?;
        Ok(())
    }

    pub fn enforce_quota(&self) -> Result<(), Error> {
        let rows = self
            .pool
            .prep_exec("SELECT name FROM quotas WHERE bytes > quota", ())?;
        for row in rows {
            let row = row.map_err(Error::MySQL)?;
            let row = mysql::from_row_opt::<String>(row)?;
            println!("db over quota: {}", row);
        }
        Ok(())
    }
}
