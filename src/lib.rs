
use std::{io::Error, pin::Pin};

tonic::include_proto!("goords");

use log::error;

use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
// use game;
use tonic::{Request, Response, Status, Streaming};
use crate::game_server::Game;
pub struct  Mon{

}
impl Mon {
    pub fn new ()->Self{
        Self{}
    }
}

#[tonic::async_trait]
impl Game for Mon {
    // type MessageFromServer = MessageFromServer;
    type gameStream=Pin<Box<dyn Stream<Item = Result<MessageFromServer, tonic::Status>>+ Send + Sync + 'static>>;
    async fn register(&self,
        request:Request<RegisterRequest>,
    )->Result<Response<RegisterResponce>,Status>{
        // let a=request.into_inner();
    
        let res:Result<String, Error> = Ok(String::from("value"));
        Ok(Response::new(
        match &res {
            Ok(u)=>{
                RegisterResponce{
                    token:u.to_string()
                }
            },
            Err(e)=>{
                error!("error in register");
                RegisterResponce{
                    token:String::from("")
                }
            }
            
        }
        ))
    }

    async fn game(&self,
        request: Request<Streaming<Command>>
    )-> Result<Response<Self::gameStream>, Status>{
        // let a=request.into_inner();

        let (tx, mut rx) = mpsc::channel(4);
        // let a = request.remote_addr();
        let mut planets: Vec<MessageFromServer> = vec![]; //= persistence::repository::get_all(&get_connection(&self.pool))
        planets.push(MessageFromServer{ test_oneof: Some(message_from_server::TestOneof::Name(CardOnTableCreature{jija: String::from("Khjoireg1")}))});
        planets.push(MessageFromServer{ test_oneof: Some(message_from_server::TestOneof::Name(CardOnTableCreature{jija: String::from("Khjoireg2")}))});
        planets.push(MessageFromServer{ test_oneof: Some(message_from_server::TestOneof::SubMessage(GetCardFromDeck{jipp:String::from("Jipp1") ,joja:String::from("Joja1")}))});
        planets.push(MessageFromServer{ test_oneof: Some(message_from_server::TestOneof::SubMessage(GetCardFromDeck{jipp:String::from("Jipp2") ,joja:String::from("Joja2")}))});
        // match request{}
        
            // .expect("Can't load planets")
            // .into_iter()
            // .map(|p| {
            //     PlanetWrapper {
            //         planet: p.0,
            //         satellites: p.1,
            //     }
            //     .into()
            // })
            // .collect();

        tokio::spawn(async move {
            let mut stream = tokio_stream::iter(&planets);

            while let Some(planet) = stream.next().await {
                tx.send(Ok(MessageFromServer {test_oneof: Some(planet.test_oneof.clone().unwrap()) }
                ))
                .await
                .unwrap();
            }
        });
        
        // tokio::spawn(async move {
        //     loop {
        //         match rx.try_recv() {
        //             Ok(_) => todo!(),
        //             Err(_) => todo!(),
        //         }
                          
                
        //     }
        //     // let mut stream = tokio_stream::iter(&planets);

        //     // while let Some(planet) = stream.next().await {
        //     //     tx.send(Ok(MessageFromServer {test_oneof: Some(planet.test_oneof.clone().unwrap()) }
        //     //     ))
        //     //     .await
        //     //     .unwrap();
        //     // }
        // });
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}

pub fn pivo(){
    
    println!("Hello, world!");
}
