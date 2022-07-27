use sqlx::{postgres::PgConnection, Connection, Postgres};
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
struct User{
    id:String,
    nick_name:String,
    age:i16,
    create_time:chrono::NaiveDateTime,
    height:f32
}


//创建单链接
async fn sql_connect() -> Option<PgConnection>{
    let connect = PgConnection::connect("postgres://postgres:joyspace2_pwd@localhost:5432/test").await;
    match connect {
        Ok(conn) => {
            Some(conn)
        },
        Err(err) => {
            println!("err message: {:?}",err);
            None
        }
    }
}

//查询
async fn _select_user(connect: &mut PgConnection){
    let res = sqlx::query_as::<Postgres,User>("select id,nick_name,age,create_time,height from sys_user").fetch_all(connect).await;
    match res {
        Ok(result) => {
            println!("{:?}",result);
        },
        Err(err) => {
            println!("err = {:?}",err);
        }
    }
}

//新增
async fn _insert_user(connect: &mut PgConnection,user: User){

    let sql = sqlx::query::<Postgres>("insert into sys_user (id,nick_name,age,create_time,height) values ( $1,$2,$3,$4,$5 )")
        .bind(user.id)
        .bind(user.nick_name)
        .bind(user.age)
        .bind(user.create_time)
        .bind(user.height)
        .execute(connect).await;
    match sql {
        Ok(result) => {
            println!("{:?}",result);
        },
        Err(err) => {
            println!("{:?}",err);
        }
    }
}

//删除
async fn _delete_user(connect: &mut PgConnection,id:String){
    let res = sqlx::query("delete from sys_user where id = $1").bind(id).execute(connect).await;
    match res {
        Ok(result) => {
            println!("{:?}",result);
        },
        Err(err) => {
            println!("err= {:?}",err);
        }
    }
}

//修改
async fn update_user(connect: &mut PgConnection,nick_name:String,id:String){
    let res = sqlx::query("update sys_user set nick_name = $1 where id = $2")
        .bind(nick_name)
        .bind(id)
        .execute(connect).await;

    match res {
        Ok(result) => {
            println!("{:?}",result);
        },
        Err(err) => {
            println!("{:?}",err);
        }
    }
}


#[tokio::main]
async fn main() {
    let user = User {
        id:"".to_string(),
        nick_name:"".to_string(),
        age:19,
        create_time:chrono::NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        height:175.1
    };
    let connect = sql_connect().await;
    if let Some(mut conn) = connect {
        //增加
        update_user(&mut conn, "新昵称".to_string(), "123".to_string()).await;
    }
    println!("Hello, world!");
}
