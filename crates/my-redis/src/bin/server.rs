use mini_redis::Frame;
use my_redis::{Connection, Db};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    // let subscriber = console_subscriber::init();
    // tracing::subscriber::set_global_default(subscriber)?;

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    println!("Listening");

    let db = Db::new(10);

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();

        println!("Accepted");

        tokio::spawn(async move {
            let _ = process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) -> mini_redis::Result<()> {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {
        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                db.insert(cmd.key(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => db.get(cmd.key()).map_or_else(|| Frame::Null, Frame::Bulk),
            cmd => return Err(format!("unimplemented {cmd:?}").into()),
        };

        connection.write_frame(&response).await?;
    }
    Ok(())
}
