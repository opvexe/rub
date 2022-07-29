// 构造器模式
type Ms = u32;

#[derive(Debug)]
pub struct TLSCert{
    key: String,
    cert: String,
}

#[derive(Debug)]
pub struct Server {
    host: String,
    port: u16,
    hot_reload: bool,
    timeout: Ms,
}

impl Server{
    fn new(host: String,port: u16) -> ServerBuilder{
        return ServerBuilder{
            host,
            port,
            tls: None,
            hot_reload: None,
            timeout: None,
        }
    }
}

#[derive(Debug)]
pub struct ServerBuilder {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: Option<bool>,
    timeout: Option<Ms>,
}

impl ServerBuilder {
    fn tls(&mut self,tls: TLSCert) -> &mut Self{
        self.tls = Some(tls);
        return self;
    }
    fn hot_reload(&mut self,reload: bool) -> &mut Self{
        self.hot_reload = Some(reload);
        return self;
    }

    fn timeout(&mut self, timeout: Ms) -> &mut Self{
        self.timeout = Some(timeout);
        return self;
    }

    fn builder(&mut self) -> Server{
       return  Server{
          host: self.host.clone(),
           port: self.port.clone(),
           hot_reload: self.hot_reload.unwrap_or_default(),
           timeout: self.timeout.unwrap_or(2000),
        };
    }
}

fn main() {
    // to_string 和 to-owned 推荐使用to-owned
    let s = Server::new("127.0.0.1".to_owned(),8080).builder();
    println!("{:?}",s);
}
