use redis::{Commands, Connection};
use slint::platform::Key;

pub struct RedisClient {
    connection: Connection,
}

fn connect(host: String, pwd: String, port: u16, db: u16,) ->  Connection {

    // format - host:port
    let redis_host_name = host ;

    let redis_password = pwd; // 使用unwrap_or_else设置默认值为None

    // if Redis server needs secure connection
    let uri_scheme = "redis";

    // 构建连接 URL
    let redis_conn_url = if  "" != &redis_password {
        format!("{}://:{}@{}:{}/{}", uri_scheme, redis_password, redis_host_name, port, db)
    } else {
        format!("{}://{}:{}/{}", uri_scheme, redis_host_name, port, db)
    };

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}
fn set(con: &mut  Connection,key: &str, value: &str) -> redis::RedisResult<()> {
    let _ : ()  = redis::cmd("SET").arg(key).arg(value).query(con)?;
    Ok(())
}

fn len(con:&mut   Connection,key:&str) -> Result<i32, redis::RedisError> {
    let len: i32 = con.llen(key)?;
    Ok(len)
}

fn pop(con:&mut Connection,key:&str) -> Result<String, redis::RedisError> {
    let ret:String = con.lpop(key, None)?;
    Ok(ret)
}


impl RedisClient {
    pub fn new(host: String, pwd: String, port: u16, db: u16,) -> RedisClient {
        let   connection = connect(host, pwd, port, db);
        RedisClient { connection }
    }

    pub fn set_sting_value(&mut self, key: &str, value: &str) {
       set(&mut self.connection, key, value).unwrap()
    }

}


pub  struct RedList  {

}
// 实现迭代器
pub struct RedisListIterator {
    inner_iter: std::vec::IntoIter<String>,
}

impl Iterator for RedisListIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}
impl RedList  {
    pub fn new(  ) -> RedList  {
        RedList {   }
    }
    pub fn pop( &self,con:&mut RedisClient,key: &str)-> Result<String, redis::RedisError>  {
        let ret:String = con.connection.lpop(key , None)?;
        Ok(ret)
    }
    pub fn append( &self,con:&mut RedisClient,key: &str, value: &str) -> redis::RedisResult<()> {
        let _ : ()  =con.connection.rpush(key,value)?;
        Ok(())
    }


    pub fn size(&self,con:&mut RedisClient,key: &str)-> Result<i32, redis::RedisError> {
        let len: i32 = con.connection.llen(key)?;
        Ok(len)
    }

    // 添加一个方法用于迭代列表中的元素
    pub fn iter(&self,con:&mut RedisClient, key: &str) -> Result<RedisListIterator, redis::RedisError> {
        let list_items: Vec<String> = con.connection.lrange(key, 0, -1)?;

        Ok(RedisListIterator {
            inner_iter: list_items.into_iter(),
        })
    }

}